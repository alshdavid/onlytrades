resource "random_string" "random" {
  length = 6
  special = false
  upper = false
} 

variable "bucket_name" {
  type = string
  default = "alshdavid-onlytrades-web"
}

variable "project_name" {
  type = string
  default = "alshdavid-onlytrades"
}

