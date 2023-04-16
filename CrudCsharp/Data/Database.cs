using System.Collections.Generic;
using System.IO;
using Newtonsoft.Json;

public static class Database
{
    private const string FILE_PATH = "database.json";

    public static List<User> Read()
    {
        if (!File.Exists(FILE_PATH))
        {
            // Se o arquivo não existe, retorna uma lista vazia
            return new List<User>();
        }

        string json = File.ReadAllText(FILE_PATH);
        return JsonConvert.DeserializeObject<List<User>>(json) ?? new List<User>();
    }

    public static void Write(List<User> users)
    {
        string json = JsonConvert.SerializeObject(users, Formatting.Indented);
        File.WriteAllText(FILE_PATH, json);
    }
}
