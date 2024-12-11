locals {
  s3_origin_id = var.bucket_name
  lambda_origin_id = "API"
}

resource "aws_cloudfront_origin_access_identity" "bucket_oai" {}

resource "aws_cloudfront_distribution" "website_cloudfront" {
  enabled = true
  wait_for_deployment = false

  origin {
    domain_name = aws_s3_bucket_website_configuration.s3_website.website_endpoint
    origin_id   = var.bucket_name
    
    custom_origin_config {
      http_port = "80"
      https_port = "443"
      origin_protocol_policy = "http-only"
      origin_ssl_protocols = ["TLSv1.2"]
    }
  }

  origin {
    domain_name              = "${aws_apigatewayv2_stage.prod.api_id}.execute-api.${data.aws_region.current.name}.amazonaws.com"
    origin_id                = local.lambda_origin_id

    custom_origin_config {
      http_port              = "80"
      https_port             = "443"
      origin_protocol_policy = "https-only"
      origin_ssl_protocols   = ["TLSv1.2"]
    }
  }

  default_cache_behavior {
    target_origin_id       = local.s3_origin_id
    viewer_protocol_policy = "redirect-to-https"
    allowed_methods        = ["GET", "HEAD"]
    cached_methods         = ["GET", "HEAD"]
    cache_policy_id        = "658327ea-f89d-4fab-a63d-7e88639e58f6" # CachingOptimized
  }

  ordered_cache_behavior {
    target_origin_id          = local.lambda_origin_id
    path_pattern              = "/api/*"
    viewer_protocol_policy    = "https-only"
    allowed_methods           = ["GET", "HEAD", "OPTIONS", "PUT", "POST", "PATCH", "DELETE"]
    cached_methods            = ["GET", "HEAD"]
    cache_policy_id           = "4135ea2d-6df8-44a3-9df3-4b5a84be39ad" # CachingDisabled
    origin_request_policy_id  = "b689b0a8-53d0-40ab-baf2-68738e2966ac" # AllViewerExceptHostHeader
  }

  viewer_certificate {
    cloudfront_default_certificate = true
  }

  is_ipv6_enabled = false

  restrictions {
    geo_restriction {
      restriction_type = "none"
    }
  }
}

resource null_resource cache_invalidation {
  # prevent invalidating cache before new s3 file is uploaded
  depends_on = [
    aws_cloudfront_distribution.website_cloudfront,
    aws_s3_object.upload_object,
  ]

  # for_each = fileset("${local.upload_directory}/", "**/*.*")

  # triggers = {
  #   hash = filemd5("${local.upload_directory}/${each.value}")
  # }

  provisioner local-exec {
    # sleep is necessary to prevent throttling when invalidating many files
    # possible way of dealing with parallelism (though would lose the indiviual triggers): https://discuss.hashicorp.com/t/specify-parallelism-for-null-resource/20884/2
    command = "sleep 1; aws cloudfront create-invalidation --distribution-id ${aws_cloudfront_distribution.website_cloudfront.id} --paths '/*'"
  }
}

output "website_url" {
  value = "http://${aws_cloudfront_distribution.website_cloudfront.domain_name}"
}