require 'json'

class Database
  def self.read
    file = File.read(Rails.root.join('database.json'))
    JSON.parse(file)
  end

  def self.write(database)
    File.write(Rails.root.join('database.json'), JSON.pretty_generate(database))
  end
end
