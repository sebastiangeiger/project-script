require 'json'
require 'pry'

RSpec.configure do |config|
  config.before(:all) do
    raise "`cargo build` failed" unless system("cargo build")
  end

  config.around do |example|
    FileUtils.rm_rf("spec/sandbox")
    FileUtils.mkdir("spec/sandbox")
    example.run
    FileUtils.rm_rf("spec/sandbox")
  end
end

def run(command)
  matches = command.match(/^project\ (.*)$/)
  raise unless matches
  `target/debug/project-script #{matches[1]} --config=spec/sandbox/config 2>&1`
end

def configure
  File.open("spec/sandbox/config", "w+") do |file|
    file.write(JSON.dump(yield))
  end
end

def state_file
  JSON.parse(IO.read("spec/sandbox/state"))
end

def create_git_repo(path, options={})
  FileUtils.mkdir_p(path)
  command = [%{cd "#{path}"}, "git init ."]
  if remotes = options[:remotes]
    remotes.each_pair do |name, url|
      command += ["git remote add #{name} #{url}"]
    end
  end
  output = `#{command.join(" && ")}`
  $stderr.puts(output) unless $?.success?
end
