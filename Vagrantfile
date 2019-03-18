# -*- mode: ruby -*-
# vi: set ft=ruby :


Vagrant.configure("2") do |config|
  config.vm.box = "archlinux/archlinux"

  # For cromwell sandbox
  config.vm.provision "shell", inline: <<-SHELL
    pacman -S base-devel rustup --noconfirm
    echo kernel.unprivileged_userns_clone=1 > /etc/sysctl.d/50-user-namespaces.conf
    sysctl kernel.unprivileged_userns_clone=1
    sudo -u vagrant rustup install stable
    sudo -u vagrant rustup default stable
    sudo -u vagrant cargo install cromwell
  SHELL
end
