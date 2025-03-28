use crate::application::dtos::{ClientDto, DtoList};
use crate::application::requests::{
    CreateClientUseCaseRequest, EditClientUseCaseRequest, GetClientUseCaseRequest,
};
use crate::domain::entities::Client;
use crate::domain::repositories::ClientRepository;
use std::rc::Rc;

pub struct CreateClientUseCaseHandler {
    client_repository: Rc<dyn ClientRepository>,
}

impl CreateClientUseCaseHandler {
    pub fn new(client_repository: Rc<dyn ClientRepository>) -> CreateClientUseCaseHandler {
        CreateClientUseCaseHandler { client_repository }
    }

    pub fn execute(&self, request: CreateClientUseCaseRequest) {
        let id = self.client_repository.next_identity();
        let client = Client::new(
            id.as_str(),
            request.name.as_str(),
            request.location.as_str(),
        );

        self.client_repository.save(client);
    }
}

pub struct GetClientUseCaseHandler {
    client_repository: Rc<dyn ClientRepository>,
}

impl GetClientUseCaseHandler {
    pub fn new(client_repository: Rc<dyn ClientRepository>) -> GetClientUseCaseHandler {
        GetClientUseCaseHandler { client_repository }
    }

    pub fn execute(&self, request: GetClientUseCaseRequest) -> Result<ClientDto, String> {
        let client = self.client_repository.by_id(request.client_id.as_str())?;

        Ok(ClientDto::from_entity(&client))
    }
}

pub struct GetAllClientsUseCaseHandler {
    client_repository: Rc<dyn ClientRepository>,
}

impl GetAllClientsUseCaseHandler {
    pub fn new(client_repository: Rc<dyn ClientRepository>) -> GetAllClientsUseCaseHandler {
        GetAllClientsUseCaseHandler { client_repository }
    }

    pub fn execute(&self) -> DtoList<ClientDto> {
        DtoList(
            self.client_repository
                .all()
                .iter()
                .map(ClientDto::from_entity)
                .collect(),
        )
    }
}

pub struct EditClientUseCaseHandler {
    client_repository: Rc<dyn ClientRepository>,
}

impl EditClientUseCaseHandler {
    pub fn new(client_repository: Rc<dyn ClientRepository>) -> EditClientUseCaseHandler {
        EditClientUseCaseHandler { client_repository }
    }

    pub fn execute(&self, request: EditClientUseCaseRequest) -> Result<(), String> {
        let mut client = self.client_repository.by_id(request.client_id.as_str())?;
        client.edit(request.name.as_str(), request.location.as_str());

        self.client_repository.save(client);

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::application::dtos::ClientDto;
    use crate::application::handlers::EditClientUseCaseHandler;
    use crate::application::requests::EditClientUseCaseRequest;
    use crate::domain::entities::Client;
    use crate::domain::repositories::MockClientRepository;
    use crate::{
        CreateClientUseCaseHandler, CreateClientUseCaseRequest, GetAllClientsUseCaseHandler,
        GetClientUseCaseHandler, GetClientUseCaseRequest,
    };
    use fake::faker::address::en::CityName;
    use fake::faker::lorem::en::Sentence;
    use fake::faker::name::en::Name;
    use fake::{Fake, Faker};
    use std::rc::Rc;

    #[test]
    fn create_client_use_case_handler_execute() {
        let id: String = Faker.fake();

        let mut client_repository_mock = MockClientRepository::new();

        client_repository_mock
            .expect_next_identity()
            .times(1)
            .return_const(id.clone());

        client_repository_mock
            .expect_save()
            .withf(move |client: &Client| client.id.eq(&id))
            .times(1)
            .return_const(());

        let create_client_use_case_handler =
            CreateClientUseCaseHandler::new(Rc::new(client_repository_mock));

        create_client_use_case_handler.execute(CreateClientUseCaseRequest::new(
            Name().fake::<String>().as_str(),
            CityName().fake::<String>().as_str(),
        ));
    }

    #[test]
    fn get_client_use_case_handler_execute_ok() {
        let client: Client = Faker.fake();
        let client_expectation = ClientDto::from_entity(&client);
        let id = client.id.clone();
        let id2 = client.id.clone();

        let mut client_repository_mock = MockClientRepository::new();

        client_repository_mock
            .expect_by_id()
            .withf(move |x: &str| x.eq(id.as_str()))
            .times(1)
            .return_const(Ok(client));

        let get_client_use_case_handler =
            GetClientUseCaseHandler::new(Rc::new(client_repository_mock));

        let result =
            get_client_use_case_handler.execute(GetClientUseCaseRequest::new(id2.as_str()));

        assert_eq!(result, Ok(client_expectation));
    }

    #[test]
    fn get_client_use_case_handler_execute_ko() {
        let id: String = Faker.fake();
        let id2 = id.clone();
        let mut client_repository_mock = MockClientRepository::new();

        let error_txt: String = Sentence(3..8).fake();

        client_repository_mock
            .expect_by_id()
            .withf(move |x: &str| x.eq(id.as_str()))
            .times(1)
            .return_const(Err(error_txt.clone()));

        let get_client_use_case_handler =
            GetClientUseCaseHandler::new(Rc::new(client_repository_mock));

        let result =
            get_client_use_case_handler.execute(GetClientUseCaseRequest::new(id2.as_str()));

        assert_eq!(result, Err(error_txt))
    }

    #[test]
    fn get_all_clients_use_case_handler_execute() {
        let mut client_repository_mock = MockClientRepository::new();
        let clients: Vec<Client> = vec![Faker.fake(), Faker.fake(), Faker.fake()];
        let client_dtos: Vec<ClientDto> = clients.iter().map(ClientDto::from_entity).collect();

        client_repository_mock
            .expect_all()
            .times(1)
            .return_const(clients);

        let get_all_clients_use_case_handler =
            GetAllClientsUseCaseHandler::new(Rc::new(client_repository_mock));
        let result = get_all_clients_use_case_handler.execute();

        assert_eq!(client_dtos, result.0);
    }

    #[test]
    fn edit_client_use_case_handler_execute_ok() {
        let client: Client = Faker.fake();

        let id = client.id.clone();
        let id2 = id.clone();
        let id3 = id.clone();

        let new_name: String = Faker.fake();
        let new_location: String = Faker.fake();

        let expected_client = Client::new(id2.as_str(), new_name.as_str(), new_location.as_str());

        let mut client_repository_mock = MockClientRepository::new();

        client_repository_mock
            .expect_by_id()
            .withf(move |x: &str| x.eq(id.as_str()))
            .times(1)
            .return_const(Ok(client));

        client_repository_mock
            .expect_save()
            .withf(move |x: &Client| x == &expected_client)
            .times(1)
            .return_const(());

        let edit_client_use_case_handler =
            EditClientUseCaseHandler::new(Rc::new(client_repository_mock));

        let result = edit_client_use_case_handler.execute(EditClientUseCaseRequest::new(
            id3.as_str(),
            new_name.as_str(),
            new_location.as_str(),
        ));

        assert_eq!(Ok(()), result);
    }

    #[test]
    fn edit_client_use_case_handler_execute_on_non_existing_client() {
        let client: Client = Faker.fake();

        let id = client.id.clone();
        let id3 = id.clone();

        let new_name: String = Faker.fake();
        let new_location: String = Faker.fake();

        let error_txt: String = Sentence(3..8).fake();

        let mut client_repository_mock = MockClientRepository::new();

        client_repository_mock
            .expect_by_id()
            .withf(move |x: &str| x.eq(id.as_str()))
            .times(1)
            .return_const(Err(error_txt.clone()));

        client_repository_mock
            .expect_save()
            .times(0)
            .return_const(());

        let edit_client_use_case_handler =
            EditClientUseCaseHandler::new(Rc::new(client_repository_mock));

        let result = edit_client_use_case_handler.execute(EditClientUseCaseRequest::new(
            id3.as_str(),
            new_name.as_str(),
            new_location.as_str(),
        ));

        assert_eq!(Err(error_txt), result);
    }
}
