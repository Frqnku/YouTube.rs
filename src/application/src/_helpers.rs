use domain::_shared::DomainError;
use uuid::Uuid;

pub fn parse_uuid(id: &str, field_name: &str) -> anyhow::Result<Uuid> {
	Uuid::parse_str(id)
		.map_err(|_| DomainError::BadRequest(format!("Invalid {field_name}")))
		.map_err(Into::into)
}