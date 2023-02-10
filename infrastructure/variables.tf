variable "region" {
  description = "Region where your AWS resource"
  type        = string
  default     = "ap-northeast-1"
}

variable "aws_profile" {
  description = "your profile name"
  type        = string
}

variable "acm_certificate_arn" {
  type = string
}

variable "cloudfront_arn" {
  type = string
}
