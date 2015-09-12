RSpec.configure do |config|
  config.before(:all) do
    raise "`cargo build` failed" unless system("cargo build")
  end
end

def run(command)
  matches = command.match(/^project\ (.*)$/)
  raise unless matches
  `target/debug/project-script #{matches[1]} 2>&1`
end
