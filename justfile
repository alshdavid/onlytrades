run:
  env \
    CLIENT_PATH="{{ justfile_directory() }}/packages/client/dist" \
    cargo run --package httpd

build:
  cargo build --release

build-client:
  mkdir -p {{ justfile_directory() }}/dist
  cd packages/client && npx rspack build  
  cp -r {{ justfile_directory() }}/packages/client/dist {{ justfile_directory() }}/dist/client

fmt:
  cargo +nightly fmt

build-lambda:
  cargo build --release
  mkdir dist
  cd target/x86_64-unknown-linux-musl/release && \
  rm -rf bootstrap && \
  mv lambda {{ justfile_directory() }}/dist/bootstrap

# ARM openssl does not cross compile
# build-publish:
#   rm -rf {{ justfile_directory() }}/dist
#   mkdir -p {{ justfile_directory() }}/dist
#   cd packages/client && rm -rf dist && npx rspack build --mode production
#   cp -r {{ justfile_directory() }}/packages/client/dist {{ justfile_directory() }}/dist/client

#   env \
#     CC=aarch64-linux-gnu-gcc \
#     cargo build --package lambda --release --target aarch64-unknown-linux-musl

#   mkdir -p {{ justfile_directory() }}/dist/lambda
#   mv {{ justfile_directory() }}/target/aarch64-unknown-linux-musl/release/lambda {{ justfile_directory() }}/dist/lambda/bootstrap

build-publish:
  rm -rf {{ justfile_directory() }}/dist
  mkdir -p {{ justfile_directory() }}/dist
  cd packages/client && rm -rf dist && npx rspack build --mode production
  cp -r {{ justfile_directory() }}/packages/client/dist {{ justfile_directory() }}/dist/client

  env \
    cargo build --package lambda --release --target x86_64-unknown-linux-musl

  mkdir -p {{ justfile_directory() }}/dist/lambda
  mv {{ justfile_directory() }}/target/x86_64-unknown-linux-musl/release/lambda {{ justfile_directory() }}/dist/lambda/bootstrap

deploy:
  cd .terraform && \
  terraform init && \
  terraform validate && \
  terraform apply -auto-approve && \
  aws cloudfront create-invalidation --distribution-id $(terraform output -raw cloudfront_id) --paths '/*'
