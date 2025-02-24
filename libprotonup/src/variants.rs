use super::constants::*;
use std::{fmt, str::FromStr};

/// Struct used to build GitHub api request URLs.
/// Contains the GitHub URL, username for GE, the repository name for either Wine GE or Proton GE, and a Variant Enum for identifying the parameters type
pub struct VariantGithubParameters {
    /// this is a link back to the enum variant
    variant_ref: Variant,
    /// URL of the repository server (GitHub compatible URL only at the moment)
    pub repository_url: String,
    /// GitHub account for the variant
    pub repository_account: String,
    /// name of the repository
    pub repository_name: String,
}

impl VariantGithubParameters {
    /// new_custom is a generator for custom VariantParameters
    pub fn new_custom(
        variant: Variant,
        repository_url: String,
        repository_account: String,
        repository_name: String,
    ) -> VariantGithubParameters {
        VariantGithubParameters {
            variant_ref: variant,
            repository_url,
            repository_account,
            repository_name,
        }
    }

    ///
    pub fn variant_type(&self) -> &Variant {
        &self.variant_ref
    }
}

/// Variant is an enum with all supported "Proton" versions
#[derive(Debug, Clone)]
pub enum Variant {
    GEProton,
    WineGE,
}

impl fmt::Display for Variant {
    /// returns a string representation of this variant
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Variant::GEProton => write!(f, "GEProton"),
            Variant::WineGE => write!(f, "WineGE"),
        }
    }
}

impl FromStr for Variant {
    type Err = ();
    fn from_str(input: &str) -> Result<Variant, Self::Err> {
        match input {
            "GEProton" => Ok(Variant::GEProton),
            "WineGE" => Ok(Variant::WineGE),
            _ => Err(()),
        }
    }
}

impl Variant {
    /// returns the application target for the Variant. Steam and Lutris are the current options
    pub fn intended_application(&self) -> &str {
        match self {
            Variant::GEProton => "Steam",
            Variant::WineGE => "Lutris",
        }
    }

    /// Returns the default parameters for this Variant, used to build the GitHub URL
    pub fn get_github_parameters(&self) -> VariantGithubParameters {
        match self {
            Variant::GEProton => VariantGithubParameters {
                variant_ref: Variant::GEProton,
                repository_url: GITHUB_URL.to_owned(),
                repository_name: GEPROTON_GITHUB_REPO.to_owned(),
                repository_account: GE_GITHUB_ACCOUNT.to_owned(),
            },
            Variant::WineGE => VariantGithubParameters {
                variant_ref: Variant::WineGE,
                repository_url: GITHUB_URL.to_owned(),
                repository_name: WINEGE_GITHUB_REPO.to_owned(),
                repository_account: GE_GITHUB_ACCOUNT.to_owned(),
            },
        }
    }
}

// ALL_VARIANTS is a shorthand to all app variants
pub static ALL_VARIANTS: &[Variant] = &[Variant::GEProton, Variant::WineGE];
