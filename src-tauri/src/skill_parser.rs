//! Skill parser: reads a `SKILL.md` and extracts the skill's name and Description
//! from its YAML frontmatter.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SkillInfo {
    pub name: String,
    pub description: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseError {
    MissingFrontmatter,
    MalformedFrontmatter,
}

#[derive(serde::Deserialize)]
struct Frontmatter {
    name: String,
    description: String,
}

pub fn parse_skill(contents: &str) -> Result<SkillInfo, ParseError> {
    let contents = contents.strip_prefix('\u{feff}').unwrap_or(contents);
    let rest = contents.strip_prefix("---").ok_or(ParseError::MissingFrontmatter)?;
    let end = rest.find("\n---").ok_or(ParseError::MissingFrontmatter)?;
    let fm: Frontmatter = serde_yaml_ng::from_str(&rest[..end])
        .map_err(|_| ParseError::MalformedFrontmatter)?;
    Ok(SkillInfo { name: fm.name, description: fm.description })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ok(name: &str, description: &str) -> Result<SkillInfo, ParseError> {
        Ok(SkillInfo { name: name.into(), description: description.into() })
    }

    #[test]
    fn parses_skill_md_frontmatter() {
        let cases: &[(&str, &str, Result<SkillInfo, ParseError>)] = &[(
            "minimal frontmatter",
            "---\nname: tdd\ndescription: Test-driven development.\n---\n\n# TDD\n",
            ok("tdd", "Test-driven development."),
        ), (
            "multi-line folded description",
            "---\nname: wizard\ndescription: >\n  Generate an interactive wizard.\n  Use when provisioning.\n---\n",
            ok("wizard", "Generate an interactive wizard. Use when provisioning."),
        ), (
            "multi-line plain description",
            "---\nname: wizard\ndescription: Generate an interactive wizard.\n  Use when provisioning.\n---\n",
            ok("wizard", "Generate an interactive wizard. Use when provisioning."),
        ), (
            "CRLF line endings",
            "---\r\nname: tdd\r\ndescription: Test-driven development.\r\n---\r\n",
            ok("tdd", "Test-driven development."),
        ), (
            "UTF-8 BOM before frontmatter",
            "\u{feff}---\nname: tdd\ndescription: Test-driven development.\n---\n",
            ok("tdd", "Test-driven development."),
        ), (
            "no frontmatter",
            "# Just a heading\n",
            Err(ParseError::MissingFrontmatter),
        ), (
            "unterminated frontmatter",
            "---\nname: tdd\ndescription: never closed\n",
            Err(ParseError::MissingFrontmatter),
        ), (
            "invalid YAML",
            "---\nname: [unclosed\ndescription: x\n---\n",
            Err(ParseError::MalformedFrontmatter),
        ), (
            "missing name",
            "---\ndescription: no name here\n---\n",
            Err(ParseError::MalformedFrontmatter),
        ), (
            "missing description",
            "---\nname: tdd\n---\n",
            Err(ParseError::MalformedFrontmatter),
        )];
        for (label, input, expected) in cases {
            assert_eq!(&parse_skill(input), expected, "case: {label}");
        }
    }
}
