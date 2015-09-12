require 'json'
require 'pp'

PATHS = ["BNR", "projects"]

def project_folders(paths = PATHS)
  paths.flat_map do |path|
    path = File.absolute_path(File.join(Dir.home, path))
    Dir.glob(File.join(path, "**/.git"))
      .map {|path| File.absolute_path(File.join(path, ".."))}
  end
end

def detail_hash_for(path)
    names = `cd "#{path}" && git remote`.split("\n")
    git_remotes = names.map do |remote_name|
      `cd "#{path}" && git config --get remote.#{remote_name}.url`.strip
    end
    if git_remotes.any?
      {
        path: path,
        git_remotes: git_remotes
      }
    else
      nil
    end
end

def main
  hash = {projects: project_folders.map {|pf| detail_hash_for(pf)}.compact}
  File.open("projects.json", "w+") do |file|
    file.write(JSON.dump(hash))
  end
end

main
