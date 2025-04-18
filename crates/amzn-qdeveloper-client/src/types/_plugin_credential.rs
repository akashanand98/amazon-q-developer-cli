// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq)]
pub struct PluginCredential {
    #[allow(missing_docs)] // documentation missing in model
    pub secret_arn: ::std::option::Option<::std::string::String>,
    #[allow(missing_docs)] // documentation missing in model
    pub secret_access_role_arn: ::std::option::Option<::std::string::String>,
}
impl PluginCredential {
    #[allow(missing_docs)] // documentation missing in model
    pub fn secret_arn(&self) -> ::std::option::Option<&str> {
        self.secret_arn.as_deref()
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn secret_access_role_arn(&self) -> ::std::option::Option<&str> {
        self.secret_access_role_arn.as_deref()
    }
}
impl ::std::fmt::Debug for PluginCredential {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("PluginCredential");
        formatter.field("secret_arn", &"*** Sensitive Data Redacted ***");
        formatter.field("secret_access_role_arn", &"*** Sensitive Data Redacted ***");
        formatter.finish()
    }
}
impl PluginCredential {
    /// Creates a new builder-style object to manufacture
    /// [`PluginCredential`](crate::types::PluginCredential).
    pub fn builder() -> crate::types::builders::PluginCredentialBuilder {
        crate::types::builders::PluginCredentialBuilder::default()
    }
}

/// A builder for [`PluginCredential`](crate::types::PluginCredential).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default)]
#[non_exhaustive]
pub struct PluginCredentialBuilder {
    pub(crate) secret_arn: ::std::option::Option<::std::string::String>,
    pub(crate) secret_access_role_arn: ::std::option::Option<::std::string::String>,
}
impl PluginCredentialBuilder {
    #[allow(missing_docs)] // documentation missing in model
    pub fn secret_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.secret_arn = ::std::option::Option::Some(input.into());
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn set_secret_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.secret_arn = input;
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn get_secret_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.secret_arn
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn secret_access_role_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.secret_access_role_arn = ::std::option::Option::Some(input.into());
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn set_secret_access_role_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.secret_access_role_arn = input;
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn get_secret_access_role_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.secret_access_role_arn
    }

    /// Consumes the builder and constructs a [`PluginCredential`](crate::types::PluginCredential).
    pub fn build(self) -> crate::types::PluginCredential {
        crate::types::PluginCredential {
            secret_arn: self.secret_arn,
            secret_access_role_arn: self.secret_access_role_arn,
        }
    }
}
impl ::std::fmt::Debug for PluginCredentialBuilder {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("PluginCredentialBuilder");
        formatter.field("secret_arn", &"*** Sensitive Data Redacted ***");
        formatter.field("secret_access_role_arn", &"*** Sensitive Data Redacted ***");
        formatter.finish()
    }
}
