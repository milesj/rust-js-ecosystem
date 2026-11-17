use version_spec::{Clause, Op, Range, Requirement, SpecError, Version};

// Parses a range with npm's semantics, which differ from version_spec's: a full
// version without an operator is an exact match, like the `1.2.3` in `1.2.3 || 2`,
// instead of `~`, and a bounded range may use partial versions, like `1.2 - 2`
pub(crate) fn parse_range(value: &str) -> Result<Range, SpecError> {
    let value = value
        .split("||")
        .map(|clause| {
            // A bounded range requires full versions, while npm also
            // supports partial versions, like `1.2 - 2`, so convert those
            if let Some((start, stop)) = clause.split_once(" - ") {
                let (start, stop) = (start.trim(), stop.trim());

                return if Version::parse(start).is_ok() && Version::parse(stop).is_ok() {
                    format!("{start} - {stop}")
                } else {
                    format!(">={start} <={stop}")
                };
            }

            let mut reqs = vec![];
            let mut op = String::new();

            for part in clause.split([' ', ',']) {
                if part.is_empty() || part == "&&" {
                    continue;
                }

                // An operator may be separated from its version, like `>= 1.2.3`
                if part
                    .chars()
                    .all(|ch| matches!(ch, '<' | '>' | '=' | '~' | '^' | '!'))
                {
                    op.push_str(part);
                    continue;
                }

                if op.is_empty() && Version::parse(part).is_ok() {
                    reqs.push(format!("={part}"));
                } else {
                    reqs.push(format!("{op}{part}"));
                }

                op.clear();
            }

            // A dangling operator fails to parse
            if !op.is_empty() {
                reqs.push(op);
            }

            reqs.join(" ")
        })
        .collect::<Vec<_>>()
        .join(" || ");

    Range::parse(value)
}

// An exact match on a full version is written without an operator,
// as it was most likely parsed without one, like `1.2.3 || 2`
fn format_requirement(req: &Requirement) -> String {
    let value = req.to_string();

    match value.strip_prefix('=') {
        Some(version) if req.op == Op::Exact && req.patch.is_some() => version.to_owned(),
        _ => value,
    }
}

// Formats a range with npm's syntax, in which requirements that must all
// match are separated by a space, instead of `&&`
// https://docs.npmjs.com/cli/v10/configuring-npm/package-json#dependencies
pub(crate) fn format_range(range: &Range) -> String {
    if range.clauses.is_empty() {
        return "*".into();
    }

    range
        .clauses
        .iter()
        .map(|clause| match clause {
            Clause::All(reqs) => reqs
                .iter()
                .map(format_requirement)
                .collect::<Vec<_>>()
                .join(" "),
            Clause::Only(req) => format_requirement(req),
            clause => clause.to_string(),
        })
        .collect::<Vec<_>>()
        .join(" || ")
}
