resource "aws_s3_bucket" "bucket" {
  bucket = var.bucket_name
  force_destroy = true
}

resource "aws_s3_bucket_website_configuration" "s3_website" {
  bucket = aws_s3_bucket.bucket.id
  
  index_document {
    suffix = "index.html"
  }

  error_document {
    key = "index.html"
  }
}


resource "aws_s3_bucket_public_access_block" "public_access_block" {
  bucket = aws_s3_bucket.bucket.id

  block_public_acls = false
  block_public_policy = false
  ignore_public_acls = false
  restrict_public_buckets = false
}

resource "aws_s3_bucket_policy" "bucket-policy" {
  depends_on = [aws_s3_bucket_public_access_block.public_access_block]
  bucket = aws_s3_bucket.bucket.id
  policy = jsonencode(
    {
      "Version" : "2012-10-17",
      "Statement" : [
        {
          "Sid" : "PublicReadGetObject",
          "Effect" : "Allow",
          "Principal" : "*",
          "Action" : "s3:GetObject",
          "Resource" : "arn:aws:s3:::${aws_s3_bucket.bucket.id}/*"
        }
      ]
    }
  )
}

locals {
  mime_types = jsondecode(file("${path.module}/mime-types.json"))
  upload_directory = "${path.module}/../dist/client"
}

resource "aws_s3_object" "upload_object" {
  for_each = fileset("${local.upload_directory}/", "**/*.*")
  bucket = aws_s3_bucket.bucket.id
  key = replace(each.value, "${local.upload_directory}/", "")
  source = "${local.upload_directory}/${each.value}"
  etag = filemd5("${local.upload_directory}/${each.value}")
  content_type = lookup(
    local.mime_types, 
    split(".", each.value)[length(split(".", each.value)) - 1],
    "",
  )
}

output "bucket_url" {
  value = "http://${aws_s3_bucket_website_configuration.s3_website.website_endpoint}"
}