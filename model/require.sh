!/bin/bash
# This script is used to install the required dependencies for the project.
# It is designed to be run on a Linux system with apt package manager.
# The script will install the following packages:
sudo apt-get install -y libgtk-3-dev libwebkit2gtk-4.1-dev libayatana-appindicator3-dev librsvg2-dev libsoup-3.0-dev libjavascriptcoregtk-4.1-dev gcc gdb cmake make nodejs npm libopencv-dev clang libclang-dev fonts-noto-cjk fonts-noto-cjk-extra openssh-server curl libudev-dev gstreamer1.0-fdkaac libgstreamer1.0-dev libgstreamer-plugins-base1.0-dev gstreamer1.0-plugins-good gstreamer1.0-plugins-bad gstreamer1.0-plugins-ugly libopenblas-dev

# Remove package that conflicts with serial port
sudo apt remove brltty
sudo apt autoremove

# this is for debugging, if you don't need it, you can comment it out
sudo apt-get install -y openssh-server
sudo /etc/init.d/ssh restart
sudo systemctl enable ssh
sudo systemctl start ssh

# register the path of the libtorch library
sudo cd /etc/ld.so.conf.d
# Remember to change the path to your own
sudo echo "/usr/local/lib" >> local.conf
sudo ldconfig

export RUSTUP_DIST_SERVER="https://rsproxy.cn"
export RUSTUP_UPDATE_ROOT="https://rsproxy.cn/rustup"
curl --proto '=https' --tlsv1.2 -sSf https://rsproxy.cn/rustup-init.sh | sh