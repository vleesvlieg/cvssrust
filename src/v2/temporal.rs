use crate::common::{AsStr, Optional, ParseError};
use std::str;

#[derive(Debug, PartialEq)]
pub enum Exploitability {
    NotDefined,
    High,
    Functional,
    ProofOfConcept,
    Unproven,
}

#[derive(Debug, PartialEq)]
pub enum RemediationLevel {
    NotDefined,
    Unavailable,
    Workaround,
    TemporaryFix,
    OfficialFix,
}

#[derive(Debug, PartialEq)]
pub enum ReportConfidence {
    Unconfirmed,
    Uncorroborated,
    Confirmed,
    NotDefined,
}

impl AsStr for Exploitability {
    fn as_str(&self) -> &str {
        match self {
            Exploitability::NotDefined => "ND",
            Exploitability::High => "H",
            Exploitability::Functional => "F",
            Exploitability::ProofOfConcept => "POC",
            Exploitability::Unproven => "U",
        }
    }
}

impl str::FromStr for Exploitability {
    type Err = ParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "ND" => Ok(Exploitability::NotDefined),
            "H" => Ok(Exploitability::High),
            "F" => Ok(Exploitability::Functional),
            "POC" => Ok(Exploitability::ProofOfConcept),
            "U" => Ok(Exploitability::Unproven),
            _ => Err(ParseError::IncorrectValue),
        }
    }
}

impl Optional for Exploitability {
    fn is_undefined(&self) -> bool {
        match self {
            Exploitability::NotDefined => true,
            _ => false,
        }
    }
}

impl AsStr for RemediationLevel {
    fn as_str(&self) -> &str {
        match self {
            RemediationLevel::NotDefined => "ND",
            RemediationLevel::Unavailable => "U",
            RemediationLevel::Workaround => "W",
            RemediationLevel::TemporaryFix => "TF",
            RemediationLevel::OfficialFix => "OF",
        }
    }
}

impl str::FromStr for RemediationLevel {
    type Err = ParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "ND" => Ok(RemediationLevel::NotDefined),
            "U" => Ok(RemediationLevel::Unavailable),
            "W" => Ok(RemediationLevel::Workaround),
            "TF" => Ok(RemediationLevel::TemporaryFix),
            "OF" => Ok(RemediationLevel::OfficialFix),
            _ => Err(ParseError::IncorrectValue),
        }
    }
}

impl Optional for RemediationLevel {
    fn is_undefined(&self) -> bool {
        match self {
            RemediationLevel::NotDefined => true,
            _ => false,
        }
    }
}

impl AsStr for ReportConfidence {
    fn as_str(&self) -> &str {
        match self {
            ReportConfidence::Unconfirmed => "UC",
            ReportConfidence::Uncorroborated => "UR",
            ReportConfidence::Confirmed => "C",
            ReportConfidence::NotDefined => "ND",
        }
    }
}

impl str::FromStr for ReportConfidence {
    type Err = ParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "UC" => Ok(ReportConfidence::Unconfirmed),
            "UR" => Ok(ReportConfidence::Uncorroborated),
            "C" => Ok(ReportConfidence::Confirmed),
            "ND" => Ok(ReportConfidence::NotDefined),
            _ => Err(ParseError::IncorrectValue),
        }
    }
}

impl Optional for ReportConfidence {
    fn is_undefined(&self) -> bool {
        match self {
            ReportConfidence::NotDefined => true,
            _ => false,
        }
    }
}