export host_code_directory="/home/ubuntu/rust-cld2"
docker run --name temp_rustcld2_develop  -v $host_code_directory:/opt/rust-cld2  --net=host -d beclab/rustcld2_develop