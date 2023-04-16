using Microsoft.AspNetCore.Mvc;
using System.Collections.Generic;
using System.Linq;
using System.Net.Http;
using System.Text.Json;
using System.Threading.Tasks;

[Route("api/[controller]")]
[ApiController]
public class UsersController : ControllerBase
{
    private readonly HttpClient _httpClient;
    private const string _baseAddress = "http://localhost:5138/";

    public UsersController(IHttpClientFactory httpClientFactory)
    {
        _httpClient = httpClientFactory.CreateClient();
        _httpClient.BaseAddress = new System.Uri(_baseAddress);
    }

    // Listar todos os usuários
    [HttpGet]
    public async Task<ActionResult<List<User>>> Index()
    {
        var response = await _httpClient.GetAsync("api/Users");
        if (response.IsSuccessStatusCode)
        {
            var usersJson = await response.Content.ReadAsStringAsync();
            var users = JsonSerializer.Deserialize<List<User>>(usersJson);
            return Ok(users);
        }
        return BadRequest(new { message = "Erro ao listar usuários." });
    }

    // Buscar usuário pelo ID
    [HttpGet("{id}")]
    public async Task<ActionResult<User>> Show(int id)
    {
        var response = await _httpClient.GetAsync($"api/Users/{id}");
        if (response.IsSuccessStatusCode)
        {
            var userJson = await response.Content.ReadAsStringAsync();
            var user = JsonSerializer.Deserialize<User>(userJson);
            return Ok(user);
        }
        return NotFound(new { message = "Usuário não encontrado." });
    }

    // Criar um novo usuário
    [HttpPost]
    public async Task<ActionResult<User>> Create([FromBody] User newUser)
    {
        var payload = new StringContent(JsonSerializer.Serialize(newUser), System.Text.Encoding.UTF8, "application/json");
        var response = await _httpClient.PostAsync("api/Users", payload);
        List<User>? users = Database.Read();
            if (users == null)
                {
                 return NotFound(new { message = "Não foi possível carregar os usuários." });
                }

        if (response.IsSuccessStatusCode)
        {
            var createdUserJson = await response.Content.ReadAsStringAsync();
            var createdUser = JsonSerializer.Deserialize<User>(createdUserJson);
            return CreatedAtAction(nameof(Show), new { id = createdUser.Id }, createdUser);
        }
        return BadRequest(new { message = "Erro ao criar usuário." });
    }

    // Atualizar um usuário existente
    [HttpPut("{id}")]
    public async Task<ActionResult<User>> Update(int id, [FromBody] User updatedUser)
    {
        var payload = new StringContent(JsonSerializer.Serialize(updatedUser), System.Text.Encoding.UTF8, "application/json");
        var response = await _httpClient.PutAsync($"api/Users/{id}", payload);
        if (response.IsSuccessStatusCode)
        {
            var updatedUserJson = await response.Content.ReadAsStringAsync();
            var updatedUser2 = JsonSerializer.Deserialize<User>(updatedUserJson);
            return Ok(updatedUser2);
        }
        return BadRequest(new { message = "Erro ao atualizar usuário." });
    }

    // Deletar um usuário existente
    [HttpDelete("{id}")]
    public async Task<ActionResult> Delete(int id)
    {
        var response = await _httpClient.DeleteAsync($"api/Users/{id}");
        if (response.IsSuccessStatusCode)
        {
            return NoContent();
        }
        return BadRequest(new { message = "Erro ao deletar usuário." });
    }
}

