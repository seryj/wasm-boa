# wasm-pack test --node # this command does not work - need to fix this.
cargo test --lib
wasm-pack build --out-dir boa-web/pkg --target web --release
# docker run -it --rm -d -p 8080:80 --name web -v ~/programming/rust/wasm-boa/boa-web/:/usr/share/nginx/html .

# Start nginx web server
cd boa-web
echo $PWD
docker stop web
docker run -it --rm -d -p 8080:80 --name web -v $PWD:/usr/share/nginx/html nginx:1.19.6-alpine

# Then, the game is available at http://localhost:8080/static
