terraform {
  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "~> 4.16"
    }
  }

  required_version = ">= 1.2.0"

  backend "s3" {
    bucket                      = "terraform-backend"
    key                         = "blog-romira-dev/terraform.tfstate"
    profile                     = "oci_s3"
    region                      = "ap-tokyo-1"
    endpoint                    = "https://nr7eduszgfzb.compat.objectstorage.ap-tokyo-1.oraclecloud.com"
    skip_region_validation      = true
    skip_credentials_validation = true
    skip_metadata_api_check     = true
    force_path_style            = true
  }
}

provider "aws" {
  region  = var.region
  profile = var.aws_profile
}

