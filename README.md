# AWS Lambda + Rust Starter Project

This will provision
- AWS Lambda function
- Will provision an AWS Gateway
  - Uses a single default route so all endpoints call the same Lambda function

## Setup

Make sure you have the Terraform and AWS CLIs set up and configured on your machine, then run:

```bash
cd terraform
terraform init
terraform apply
```

## Running locally
