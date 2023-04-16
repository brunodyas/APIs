class UsersController < ApplicationController
    # Listar todos os usuários
    def index
      database = Database.read
      render json: database['users']
    end
  
    # Buscar usuário pelo ID
    def show
      database = Database.read
      user = database['users'].find { |user| user['id'] == params[:id].to_i }
      if user
        render json: user
      else
        render json: { message: 'Usuário não encontrado.' }, status: :not_found
      end
    end
  
    # Criar um novo usuário
    def create
      database = Database.read
      new_user = params.permit(:user, :password).to_h
      database['users'] << new_user
      Database.write(database) # Salvar o novo usuário no arquivo JSON
      render json: new_user, status: :created
    end
  
    # Atualizar um usuário existente
    def update
      database = Database.read
      updated_user = params.permit(:user, :password).to_h
      index = database['users'].find_index { |user| user['id'] == params[:id].to_i }
      if index
        database['users'][index] = { 'id' => params[:id].to_i, **updated_user }
        Database.write(database) # Salvar o usuário atualizado no arquivo JSON
        render json: database['users'][index]
      else
        render json: { message: 'Usuário não encontrado.' }, status: :not_found
      end
    end
  
    # Deletar um usuário existente
    def destroy
      database = Database.read
      index = database['users'].find_index { |user| user['id'] == params[:id].to_i }
      if index
        deleted_user = database['users'].delete_at(index)
        Database.write(database) # Salvar o usuário deletado no arquivo JSON
        render json: { message: 'Usuário deletado com sucesso.' }
      else
        render json: { message: 'Usuário não encontrado.' }, status: :not_found
      end
    end

    def user_params
        params.permit(:user, :password)
    end
  end
  