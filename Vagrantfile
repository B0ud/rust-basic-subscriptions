# -*- mode: ruby -*-
# vi: set ft=ruby :

unless Vagrant.has_plugin?("vagrant-docker-compose")
    system("vagrant plugin install vagrant-docker-compose")
    puts "Dependencies installed, please try the command again."
    exit
  end


Vagrant.configure("2") do |config|
    config.vm.box = "ubuntu/jammy64"
    config.vm.synced_folder '.', '/vagrant', disabled: true
    config.vm.synced_folder ".", "/home/vagrant/app", disabled: false

    config.vm.network(:forwarded_port, guest: 5432, host: 5432)

    config.vm.provision :docker
    config.vm.provision :docker_compose, yml: "/home/vagrant/app/docker-compose.yml", run: "always"
  end