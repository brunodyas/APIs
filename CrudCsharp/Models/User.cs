public class User
{
    public int Id { get; set; }
    public string Username { get; set; }
    public string Password { get; set; }

    public User()
    {
        this.Id = 0;
        this.Username = string.Empty;
        this.Password = string.Empty;
    }
}
