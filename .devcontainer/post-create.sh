curl -L \
    https://sourceforge.net/projects/freepascal/files/Linux/3.2.2/fpc-3.2.2.x86_64-linux.tar \
    > $PWD/fpc-3.2.2.x86_64-linux.tar
tar -xf $PWD/fpc-3.2.2.x86_64-linux.tar && \
rm -rf $PWD/fpc-3.2.2.x86_64-linux.tar && \
cd $PWD/fpc-3.2.2.x86_64-linux && \
echo | ./install.sh && \
cd ../ && \
rm -rf $PWD/fpc-3.2.2.x86_64-linux && \
rustup default nightly && \
cargo build --release && \
cd dss_rs_sys && \
mkdir -p $PWD/klusolvex && \
cp $PWD/klusolve/LICENSE $PWD/klusolvex/LICENSE && \
cd dss_capi && \
$PWD/build/build_linux_x64.sh && \
cp $PWD/lib/linux_x64/*.so /usr/local/lib/ && \
cd ../../ && \
cargo test -- --test-threads 1