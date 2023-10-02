from ubuntu:latest

run apt update
run apt install -y curl
run apt install -y gcc
run apt install -y git
run apt install -y htop
run apt install -y less
run apt install -y tmux
run apt install -y tree
run apt install -y vim
run apt install -y wget

# get and install dotfiles
workdir /root
run git clone https://github.com/skeggib/dotfiles.git
run rm -f .bashrc .inputrc .vimrc .gdbinit .gitconfig .tmux.conf .tmux.conf.local
run ln -s dotfiles/.bashrc .bashrc
run ln -s dotfiles/.gdbinit .gdbinit
run ln -s dotfiles/.gitconfig .gitconfig
run ln -s dotfiles/.inputrc .inputrc
run ln -s dotfiles/.tmux.conf .tmux.conf
run ln -s dotfiles/.tmux.conf.local .tmux.conf.local
run ln -s dotfiles/.vimrc .vimrc

# install rust and a linker
run curl https://sh.rustup.rs -sSf | sh -s -- -y
run apt install -y gcc
