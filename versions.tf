terraform {
  required_version = ">= 1.7.0"

  required_providers {
    aws = {
      source                = "hashicorp/aws"
      version               = ">= 5.22.0"
      configuration_aliases = [aws.ekg_api]
    }
    github = {
      source  = "integrations/github"
      version = "~> 6.2"
    }
  }
}
