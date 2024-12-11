resource "random_string" "random" {
  length = 6
  special = false
  upper = false
} 

variable "bucket_name" {
  type = string
  default = "alshdavid-templates-rust-lambda-cognito-rust"
}

variable "project_name" {
  type = string
  default = "alshdavid-templates-rust-lambda-cognito-rust"
}

