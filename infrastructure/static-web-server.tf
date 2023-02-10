locals {
  bucket_name = "blog-romira-dev-static"
}

# s3
data "aws_iam_policy_document" "allow_access_from_cloudfront" {
  statement {
    sid    = "AllowCloudFrontServicePrincipal"
    effect = "Allow"
    principals {
      type        = "Service"
      identifiers = ["cloudfront.amazonaws.com"]
    }
    actions   = ["s3:GetObject"]
    resources = ["arn:aws:s3:::${local.bucket_name}/*"]
    condition {
      test     = "StringEquals"
      variable = "AWS:SourceArn"
      values   = [var.cloudfront_arn]
    }
  }
}

resource "aws_s3_bucket_policy" "allow_access_from_cloudfront" {
  bucket = local.bucket_name
  policy = data.aws_iam_policy_document.allow_access_from_cloudfront.json
}

module "s3-bucket" {
  source  = "terraform-aws-modules/s3-bucket/aws"
  version = "3.6.1"

  bucket = local.bucket_name

  tags = {
    Production-Name = local.tag
  }
}

# cloudfront
data "aws_cloudfront_cache_policy" "cache_policy" {
  name = "Managed-CachingOptimized"
}

data "aws_cloudfront_origin_request_policy" "origin_request_policy" {
  name = "Managed-UserAgentRefererHeaders"
}

data "aws_cloudfront_response_headers_policy" "response_headers_policy" {
  name = "Managed-SimpleCORS"
}

resource "aws_cloudfront_origin_access_control" "origin_access_control" {
  name                              = local.tag
  origin_access_control_origin_type = "s3"
  signing_behavior                  = "always"
  signing_protocol                  = "sigv4"
}

resource "aws_cloudfront_distribution" "s3_distribution" {
  origin {
    domain_name              = module.s3-bucket.s3_bucket_bucket_domain_name
    origin_access_control_id = aws_cloudfront_origin_access_control.origin_access_control.id
    origin_id                = module.s3-bucket.s3_bucket_id
  }

  aliases = ["blog.romira.dev"]
  viewer_certificate {
    acm_certificate_arn      = var.acm_certificate_arn
    ssl_support_method       = "sni-only"
    minimum_protocol_version = "TLSv1.2_2021"
  }

  enabled             = true
  is_ipv6_enabled     = true
  default_root_object = "index.html"

  default_cache_behavior {
    target_origin_id = module.s3-bucket.s3_bucket_id

    compress               = true
    viewer_protocol_policy = "redirect-to-https"

    allowed_methods = ["DELETE", "GET", "HEAD", "OPTIONS", "PATCH", "POST", "PUT"]
    cached_methods  = ["GET", "HEAD"]

    cache_policy_id          = data.aws_cloudfront_cache_policy.cache_policy.id
    origin_request_policy_id = data.aws_cloudfront_origin_request_policy.origin_request_policy.id

    response_headers_policy_id = data.aws_cloudfront_response_headers_policy.response_headers_policy.id

    min_ttl     = 0
    default_ttl = 3600
    max_ttl     = 86400
  }

  price_class = "PriceClass_200"

  restrictions {
    geo_restriction {
      restriction_type = "whitelist"
      locations        = ["JP"]
    }
  }

  retain_on_delete = true

  tags = {
    Production-Name = local.tag
  }
}
