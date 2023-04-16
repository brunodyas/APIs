const express = require('express');
const fs = require('fs');
const app = express();

app.use(express.json());

// Ler o arquivo JSON do banco de dados
const readDatabase = () => {
  const rawdata = fs.readFileSync('database.json');
  return JSON.parse(rawdata);
};

// Escrever o arquivo JSON do banco de dados
const writeDatabase = (database) => {
  fs.writeFileSync('database.json', JSON.stringify(database));
};

// Listar todos os usuários
app.get('/users', (req, res) => {
  const database = readDatabase();
  res.json(database.users);
});

// Buscar usuário pelo ID
app.get('/users/:id', (req, res) => {
  const userId = parseInt(req.params.id);
  const database = readDatabase();
  const user = database.users.find(user => user.id === userId);
  if (user) {
    res.json(user);
  } else {
    res.status(404).json({ message: 'Usuário não encontrado.' });
  }
});

// Criar um novo usuário
app.post('/users', (req, res) => {
  const newUser = req.body;
  const database = readDatabase();
  newUser.id = database.users.length + 1;
  database.users.push(newUser);
  writeDatabase(database); // Salvar o novo usuário no arquivo JSON
  res.status(201).json(newUser);
});

// Atualizar um usuário existente
app.put('/users/:id', (req, res) => {
  const userId = parseInt(req.params.id);
  const updatedUser = req.body;
  const database = readDatabase();
  const index = database.users.findIndex(user => user.id === userId);
  if (index !== -1) {
    database.users[index] = { id: userId, ...updatedUser };
    writeDatabase(database); // Salvar o usuário atualizado no arquivo JSON
    res.json(database.users[index]);
  } else {
    res.status(404).json({ message: 'Usuário não encontrado.' });
  }
});

// Deletar um usuário existente
app.delete('/users/:id', (req, res) => {
  const userId = parseInt(req.params.id);
  const database = readDatabase();
  const index = database.users.findIndex(user => user.id === userId);
  if (index !== -1) {
    database.users.splice(index, 1);
    writeDatabase(database); // Salvar o usuário deletado no arquivo JSON
    res.json({ message: 'Usuário deletado com sucesso.' });
  } else {
    res.status(404).json({ message: 'Usuário não encontrado.' });
  }
});

// Iniciar o servidor
app.listen(3000, () => {
  console.log('API rodando na porta 3000.');
});
