# basic install
```
cargo install ptx-linker 

rustup override set nightly

rustup target add nvptx64-nvidia-cuda
```

# install cuda and cudnn

#check environment if compilation fails (linker can't find library)
## LD_LIBRARY_PATH !=== LIBRARY_PATH (cublas error)
```
export PATH=/usr/local/cuda/bin:$PATH
# export LD_LIBRARY_PATH=/usr/local/cuda/lib64:$LD_LIBRARY_PATH
export LD_LIBRARY_PATH=/usr/local/cuda/lib64:/usr/local/cuda-10.0/targets/x86_64-linux/lib 
export LIBRARY_PATH=$LD_LIBRARY_PATH
export CUDA_HOME=/usr/local/cuda
export CUDA_LIBRARY_PATH=/usr/local/cuda/lib
export HOST_COMPILER=/usr/bin/cuda-g++
export CPLUS_INCLUDE_PATH=$CPLUS_INCLUDE_PATH:$HOME/include:/usr/local/cuda-10.0/include
export KERNEL_PTX_PATH=/home/bumzack/stoff/rust/rust-cuda-matrix/ptx/cuda_matrix.ptx
export KERNEL_PTX_PATH_RUST_MANDEL=/home/bumzack/stoff/rust/rustmandel_cuda/ptx/cuda_kernel_mandel.ptx
export KERNEL_PTX_PATH_RUST_RENDER=/home/bumzack/stoff/rust/raytracer-challenge/ptx/cuda_kernel_raytracer.ptx
```


#  gfx crate  & metal problems 
see here for possible solutions: https://github.com/gfx-rs/gfx/issues/2309
_