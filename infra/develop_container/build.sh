develop_container_dir=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd ) 
echo $develop_container_dir
infra_dir=$(dirname -- "$develop_container_dir") 
echo $infra_dir
root_dir=$(dirname -- "$infra_dir") 
echo $root_dir
DOCKER_FILE_PATH=$develop_container_dir/Dockerfile
PREFIX=beclab

docker  build    \
    -f ${DOCKER_FILE_PATH} \
    -t ${PREFIX}/rustcld2_develop $root_dir