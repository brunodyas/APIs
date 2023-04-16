import json
from flask import Flask, jsonify, request

app = Flask(__name__)

# Ler o arquivo JSON do banco de dados
def read_database():
    with open('database.json', 'r') as f:
        return json.load(f)

# Escrever o arquivo JSON do banco de dados
def write_database(database):
    with open('database.json', 'w') as f:
        json.dump(database, f)

# Listar todos os usuários
@app.route('/users', methods=['GET'])
def get_users():
    database = read_database()
    return jsonify(database['users'])

# Buscar usuário pelo ID
@app.route('/users/<int:user_id>', methods=['GET'])
def get_user(user_id):
    database = read_database()
    user = next((user for user in database['users'] if user['id'] == user_id), None)
    if user:
        return jsonify(user)
    else:
        return jsonify({'message': 'Usuário não encontrado.'}), 404

# Criar um novo usuário
@app.route('/users', methods=['POST'])
def create_user():
    database = read_database()
    new_user = request.json
    new_user['id'] = len(database['users']) + 1
    database['users'].append(new_user)
    write_database(database) # Salvar o novo usuário no arquivo JSON
    return jsonify(new_user), 201

# Atualizar um usuário existente
@app.route('/users/<int:user_id>', methods=['PUT'])
def update_user(user_id):
    database = read_database()
    updated_user = request.json
    index = next((i for i, user in enumerate(database['users']) if user['id'] == user_id), None)
    if index is not None:
        database['users'][index] = {'id': user_id, **updated_user}
        write_database(database) # Salvar o usuário atualizado no arquivo JSON
        return jsonify(database['users'][index])
    else:
        return jsonify({'message': 'Usuário não encontrado.'}), 404

# Deletar um usuário existente
@app.route('/users/<int:user_id>', methods=['DELETE'])
def delete_user(user_id):
    database = read_database()
    index = next((i for i, user in enumerate(database['users']) if user['id'] == user_id), None)
    if index is not None:
        database['users'].pop(index)
        write_database(database) # Salvar o usuário deletado no arquivo JSON
        return jsonify({'message': 'Usuário deletado com sucesso.'})
    else:
        return jsonify({'message': 'Usuário não encontrado.'}), 404

# Iniciar o servidor
if __name__ == '__main__':
    app.run(debug=True)
