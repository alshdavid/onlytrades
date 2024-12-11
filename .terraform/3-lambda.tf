resource "aws_iam_role" "main_lambda_iam_role" {
  name = format("%s_%s", var.project_name, "function")

  assume_role_policy = <<POLICY
{
  "Version": "2012-10-17",
  "Statement": [
    {
      "Effect": "Allow",
      "Principal": {
        "Service": "lambda.amazonaws.com"
      },
      "Action": "sts:AssumeRole"
    }
  ]
}
POLICY
}

resource "aws_iam_role_policy_attachment" "main_lambda_policy" {
  role = aws_iam_role.main_lambda_iam_role.name
  policy_arn = "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole"
}

resource "aws_cloudwatch_log_group" "main_logs" {
  retention_in_days = 14
}

data "archive_file" "lambda_artifact" {
  type = "zip"
  source_dir = "${path.module}/../dist/lambda"
  output_path = "${path.module}/../dist/lambda/.lambda.zip"
}

resource "aws_s3_object" "lambda_artifact" {
  bucket = aws_s3_bucket.bucket.id
  key = ".lambda.zip"
  source = data.archive_file.lambda_artifact.output_path
  etag = filemd5(data.archive_file.lambda_artifact.output_path)
}

resource "aws_lambda_function" "main_lambda" {
  function_name = format("%s_%s", var.project_name, "function")

  source_code_hash = data.archive_file.lambda_artifact.output_base64sha256
  role = aws_iam_role.main_lambda_iam_role.arn
  
  runtime = "provided.al2"
  handler = "hello.handler"

  # architectures = ["x86_64"]
  architectures = ["arm64"]
  
  memory_size = 128

  s3_bucket = aws_s3_bucket.bucket.id
  s3_key = aws_s3_object.lambda_artifact.key

  environment {
    variables = {
      LOCAL_ORIGIN = aws_apigatewayv2_stage.prod.invoke_url
      COGNITO_SECRET = aws_cognito_user_pool_client.client.client_secret
    }
  }
}
