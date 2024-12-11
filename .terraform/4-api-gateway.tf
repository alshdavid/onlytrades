resource "aws_apigatewayv2_api" "main_api_gateway" {
  name = format("%s_%s", var.project_name, "api")
  protocol_type = "HTTP"
}

resource "aws_apigatewayv2_stage" "prod" {
  api_id  = aws_apigatewayv2_api.main_api_gateway.id
  name    = "$default"
  auto_deploy = true
}

resource "aws_apigatewayv2_integration" "api_integration" {
  api_id = aws_apigatewayv2_api.main_api_gateway.id

  payload_format_version = "2.0"
  integration_uri = aws_lambda_function.main_lambda.invoke_arn
  integration_type = "AWS_PROXY"
  integration_method = "POST"
}

resource "aws_apigatewayv2_route" "api_default" {
  api_id = aws_apigatewayv2_api.main_api_gateway.id
  route_key = "$default"
  target = "integrations/${aws_apigatewayv2_integration.api_integration.id}"
}

resource "aws_lambda_permission" "lambda_invoke" {
  statement_id   = "AllowExecutionFromApiGateway"
  action         = "lambda:InvokeFunction"
  function_name  = aws_lambda_function.main_lambda.function_name
  principal      = "apigateway.amazonaws.com"
  source_arn = "${aws_apigatewayv2_api.main_api_gateway.execution_arn}/*"
}

output "api_url" {
  value = aws_apigatewayv2_stage.prod.invoke_url
}