// This file is @generated by wasmcloud/weld-codegen 0.6.0.
// It is not intended for manual editing.
// namespace: za.co.mfrtechnologies.wasminterfaces.auth

#[allow(unused_imports)]
use async_trait::async_trait;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::{borrow::Borrow, borrow::Cow, io::Write, string::ToString};
#[allow(unused_imports)]
use wasmbus_rpc::{
    cbor::*,
    common::{
        deserialize, message_format, serialize, Context, Message, MessageDispatch, MessageFormat,
        SendOpts, Transport,
    },
    error::{RpcError, RpcResult},
    Timestamp,
};

#[allow(dead_code)]
pub const SMITHY_VERSION: &str = "1.0";

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct LoginRequest {
    #[serde(default)]
    pub password: String,
    #[serde(default)]
    pub username: String,
}

// Encode LoginRequest as CBOR and append to output stream
#[doc(hidden)]
#[allow(unused_mut)]
pub fn encode_login_request<W: wasmbus_rpc::cbor::Write>(
    mut e: &mut wasmbus_rpc::cbor::Encoder<W>,
    val: &LoginRequest,
) -> RpcResult<()>
where
    <W as wasmbus_rpc::cbor::Write>::Error: std::fmt::Display,
{
    e.map(2)?;
    e.str("password")?;
    e.str(&val.password)?;
    e.str("username")?;
    e.str(&val.username)?;
    Ok(())
}

// Decode LoginRequest from cbor input stream
#[doc(hidden)]
pub fn decode_login_request(
    d: &mut wasmbus_rpc::cbor::Decoder<'_>,
) -> Result<LoginRequest, RpcError> {
    let __result = {
        let mut password: Option<String> = None;
        let mut username: Option<String> = None;

        let is_array = match d.datatype()? {
            wasmbus_rpc::cbor::Type::Array => true,
            wasmbus_rpc::cbor::Type::Map => false,
            _ => {
                return Err(RpcError::Deser(
                    "decoding struct LoginRequest, expected array or map".to_string(),
                ))
            }
        };
        if is_array {
            let len = d.fixed_array()?;
            for __i in 0..(len as usize) {
                match __i {
                    0 => password = Some(d.str()?.to_string()),
                    1 => username = Some(d.str()?.to_string()),
                    _ => d.skip()?,
                }
            }
        } else {
            let len = d.fixed_map()?;
            for __i in 0..(len as usize) {
                match d.str()? {
                    "password" => password = Some(d.str()?.to_string()),
                    "username" => username = Some(d.str()?.to_string()),
                    _ => d.skip()?,
                }
            }
        }
        LoginRequest {
            password: if let Some(__x) = password {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field LoginRequest.password (#0)".to_string(),
                ));
            },

            username: if let Some(__x) = username {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field LoginRequest.username (#1)".to_string(),
                ));
            },
        }
    };
    Ok(__result)
}
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct Role {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: String,
}

// Encode Role as CBOR and append to output stream
#[doc(hidden)]
#[allow(unused_mut)]
pub fn encode_role<W: wasmbus_rpc::cbor::Write>(
    mut e: &mut wasmbus_rpc::cbor::Encoder<W>,
    val: &Role,
) -> RpcResult<()>
where
    <W as wasmbus_rpc::cbor::Write>::Error: std::fmt::Display,
{
    e.map(2)?;
    e.str("id")?;
    e.str(&val.id)?;
    e.str("name")?;
    e.str(&val.name)?;
    Ok(())
}

// Decode Role from cbor input stream
#[doc(hidden)]
pub fn decode_role(d: &mut wasmbus_rpc::cbor::Decoder<'_>) -> Result<Role, RpcError> {
    let __result = {
        let mut id: Option<String> = None;
        let mut name: Option<String> = None;

        let is_array = match d.datatype()? {
            wasmbus_rpc::cbor::Type::Array => true,
            wasmbus_rpc::cbor::Type::Map => false,
            _ => {
                return Err(RpcError::Deser(
                    "decoding struct Role, expected array or map".to_string(),
                ))
            }
        };
        if is_array {
            let len = d.fixed_array()?;
            for __i in 0..(len as usize) {
                match __i {
                    0 => id = Some(d.str()?.to_string()),
                    1 => name = Some(d.str()?.to_string()),
                    _ => d.skip()?,
                }
            }
        } else {
            let len = d.fixed_map()?;
            for __i in 0..(len as usize) {
                match d.str()? {
                    "id" => id = Some(d.str()?.to_string()),
                    "name" => name = Some(d.str()?.to_string()),
                    _ => d.skip()?,
                }
            }
        }
        Role {
            id: if let Some(__x) = id {
                __x
            } else {
                return Err(RpcError::Deser("missing field Role.id (#0)".to_string()));
            },

            name: if let Some(__x) = name {
                __x
            } else {
                return Err(RpcError::Deser("missing field Role.name (#1)".to_string()));
            },
        }
    };
    Ok(__result)
}
pub type Roles = Vec<Role>;

// Encode Roles as CBOR and append to output stream
#[doc(hidden)]
#[allow(unused_mut)]
pub fn encode_roles<W: wasmbus_rpc::cbor::Write>(
    mut e: &mut wasmbus_rpc::cbor::Encoder<W>,
    val: &Roles,
) -> RpcResult<()>
where
    <W as wasmbus_rpc::cbor::Write>::Error: std::fmt::Display,
{
    e.array(val.len() as u64)?;
    for item in val.iter() {
        encode_role(e, item)?;
    }
    Ok(())
}

// Decode Roles from cbor input stream
#[doc(hidden)]
pub fn decode_roles(d: &mut wasmbus_rpc::cbor::Decoder<'_>) -> Result<Roles, RpcError> {
    let __result = {
        if let Some(n) = d.array()? {
            let mut arr: Vec<Role> = Vec::with_capacity(n as usize);
            for _ in 0..(n as usize) {
                arr.push(decode_role(d).map_err(|e| {
                    format!(
                        "decoding 'za.co.mfrtechnologies.wasminterfaces.auth#Role': {}",
                        e
                    )
                })?)
            }
            arr
        } else {
            // indefinite array
            let mut arr: Vec<Role> = Vec::new();
            loop {
                match d.datatype() {
                    Err(_) => break,
                    Ok(wasmbus_rpc::cbor::Type::Break) => break,
                    Ok(_) => arr.push(decode_role(d).map_err(|e| {
                        format!(
                            "decoding 'za.co.mfrtechnologies.wasminterfaces.auth#Role': {}",
                            e
                        )
                    })?),
                }
            }
            arr
        }
    };
    Ok(__result)
}
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct User {
    #[serde(default)]
    pub address: String,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    pub created_at: Timestamp,
    #[serde(default)]
    pub email: String,
    #[serde(rename = "firstName")]
    #[serde(default)]
    pub first_name: String,
    #[serde(default)]
    pub gender: String,
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub id_number: String,
    #[serde(rename = "lastName")]
    #[serde(default)]
    pub last_name: String,
    #[serde(default)]
    pub password: String,
    #[serde(rename = "phoneNumber")]
    #[serde(default)]
    pub phone_number: String,
    #[serde(default)]
    pub role_id: String,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    pub updated_at: Timestamp,
    #[serde(default)]
    pub username: String,
}

// Encode User as CBOR and append to output stream
#[doc(hidden)]
#[allow(unused_mut)]
pub fn encode_user<W: wasmbus_rpc::cbor::Write>(
    mut e: &mut wasmbus_rpc::cbor::Encoder<W>,
    val: &User,
) -> RpcResult<()>
where
    <W as wasmbus_rpc::cbor::Write>::Error: std::fmt::Display,
{
    e.map(13)?;
    e.str("address")?;
    e.str(&val.address)?;
    e.str("createdAt")?;
    e.i64(val.created_at.sec)?;
    e.u32(val.created_at.nsec)?;
    e.str("email")?;
    e.str(&val.email)?;
    e.str("firstName")?;
    e.str(&val.first_name)?;
    e.str("gender")?;
    e.str(&val.gender)?;
    e.str("id")?;
    e.str(&val.id)?;
    e.str("id_number")?;
    e.str(&val.id_number)?;
    e.str("lastName")?;
    e.str(&val.last_name)?;
    e.str("password")?;
    e.str(&val.password)?;
    e.str("phoneNumber")?;
    e.str(&val.phone_number)?;
    e.str("role_id")?;
    e.str(&val.role_id)?;
    e.str("updatedAt")?;
    e.i64(val.updated_at.sec)?;
    e.u32(val.updated_at.nsec)?;
    e.str("username")?;
    e.str(&val.username)?;
    Ok(())
}

// Decode User from cbor input stream
#[doc(hidden)]
pub fn decode_user(d: &mut wasmbus_rpc::cbor::Decoder<'_>) -> Result<User, RpcError> {
    let __result = {
        let mut address: Option<String> = None;
        let mut created_at: Option<Timestamp> = None;
        let mut email: Option<String> = None;
        let mut first_name: Option<String> = None;
        let mut gender: Option<String> = None;
        let mut id: Option<String> = None;
        let mut id_number: Option<String> = None;
        let mut last_name: Option<String> = None;
        let mut password: Option<String> = None;
        let mut phone_number: Option<String> = None;
        let mut role_id: Option<String> = None;
        let mut updated_at: Option<Timestamp> = None;
        let mut username: Option<String> = None;

        let is_array = match d.datatype()? {
            wasmbus_rpc::cbor::Type::Array => true,
            wasmbus_rpc::cbor::Type::Map => false,
            _ => {
                return Err(RpcError::Deser(
                    "decoding struct User, expected array or map".to_string(),
                ))
            }
        };
        if is_array {
            let len = d.fixed_array()?;
            for __i in 0..(len as usize) {
                match __i {
                    0 => address = Some(d.str()?.to_string()),
                    1 => {
                        created_at = Some(wasmbus_rpc::Timestamp {
                            sec: d.i64()?,
                            nsec: d.u32()?,
                        })
                    }
                    2 => email = Some(d.str()?.to_string()),
                    3 => first_name = Some(d.str()?.to_string()),
                    4 => gender = Some(d.str()?.to_string()),
                    5 => id = Some(d.str()?.to_string()),
                    6 => id_number = Some(d.str()?.to_string()),
                    7 => last_name = Some(d.str()?.to_string()),
                    8 => password = Some(d.str()?.to_string()),
                    9 => phone_number = Some(d.str()?.to_string()),
                    10 => role_id = Some(d.str()?.to_string()),
                    11 => {
                        updated_at = Some(wasmbus_rpc::Timestamp {
                            sec: d.i64()?,
                            nsec: d.u32()?,
                        })
                    }
                    12 => username = Some(d.str()?.to_string()),
                    _ => d.skip()?,
                }
            }
        } else {
            let len = d.fixed_map()?;
            for __i in 0..(len as usize) {
                match d.str()? {
                    "address" => address = Some(d.str()?.to_string()),
                    "createdAt" => {
                        created_at = Some(wasmbus_rpc::Timestamp {
                            sec: d.i64()?,
                            nsec: d.u32()?,
                        })
                    }
                    "email" => email = Some(d.str()?.to_string()),
                    "firstName" => first_name = Some(d.str()?.to_string()),
                    "gender" => gender = Some(d.str()?.to_string()),
                    "id" => id = Some(d.str()?.to_string()),
                    "id_number" => id_number = Some(d.str()?.to_string()),
                    "lastName" => last_name = Some(d.str()?.to_string()),
                    "password" => password = Some(d.str()?.to_string()),
                    "phoneNumber" => phone_number = Some(d.str()?.to_string()),
                    "role_id" => role_id = Some(d.str()?.to_string()),
                    "updatedAt" => {
                        updated_at = Some(wasmbus_rpc::Timestamp {
                            sec: d.i64()?,
                            nsec: d.u32()?,
                        })
                    }
                    "username" => username = Some(d.str()?.to_string()),
                    _ => d.skip()?,
                }
            }
        }
        User {
            address: if let Some(__x) = address {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field User.address (#0)".to_string(),
                ));
            },

            created_at: if let Some(__x) = created_at {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field User.created_at (#1)".to_string(),
                ));
            },

            email: if let Some(__x) = email {
                __x
            } else {
                return Err(RpcError::Deser("missing field User.email (#2)".to_string()));
            },

            first_name: if let Some(__x) = first_name {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field User.first_name (#3)".to_string(),
                ));
            },

            gender: if let Some(__x) = gender {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field User.gender (#4)".to_string(),
                ));
            },

            id: if let Some(__x) = id {
                __x
            } else {
                return Err(RpcError::Deser("missing field User.id (#5)".to_string()));
            },

            id_number: if let Some(__x) = id_number {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field User.id_number (#6)".to_string(),
                ));
            },

            last_name: if let Some(__x) = last_name {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field User.last_name (#7)".to_string(),
                ));
            },

            password: if let Some(__x) = password {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field User.password (#8)".to_string(),
                ));
            },

            phone_number: if let Some(__x) = phone_number {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field User.phone_number (#9)".to_string(),
                ));
            },

            role_id: if let Some(__x) = role_id {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field User.role_id (#10)".to_string(),
                ));
            },

            updated_at: if let Some(__x) = updated_at {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field User.updated_at (#11)".to_string(),
                ));
            },

            username: if let Some(__x) = username {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field User.username (#12)".to_string(),
                ));
            },
        }
    };
    Ok(__result)
}
/// Description of Auth service
/// wasmbus.actorReceive
#[async_trait]
pub trait Auth {
    /// Registers a user
    async fn register(&self, ctx: &Context, arg: &User) -> RpcResult<User>;
    async fn login(&self, ctx: &Context, arg: &LoginRequest) -> RpcResult<String>;
    async fn get_user_role<TS: ToString + ?Sized + std::marker::Sync>(
        &self,
        ctx: &Context,
        arg: &TS,
    ) -> RpcResult<Role>;
    async fn get_roles(&self, ctx: &Context) -> RpcResult<Roles>;
    async fn init_tables(&self, ctx: &Context) -> RpcResult<bool>;
}

/// AuthReceiver receives messages defined in the Auth service trait
/// Description of Auth service
#[doc(hidden)]
#[async_trait]
pub trait AuthReceiver: MessageDispatch + Auth {
    async fn dispatch(&self, ctx: &Context, message: Message<'_>) -> Result<Vec<u8>, RpcError> {
        match message.method {
            "Register" => {
                let value: User = wasmbus_rpc::common::deserialize(&message.arg)
                    .map_err(|e| RpcError::Deser(format!("'User': {}", e)))?;

                let resp = Auth::register(self, ctx, &value).await?;
                let buf = wasmbus_rpc::common::serialize(&resp)?;

                Ok(buf)
            }
            "Login" => {
                let value: LoginRequest = wasmbus_rpc::common::deserialize(&message.arg)
                    .map_err(|e| RpcError::Deser(format!("'LoginRequest': {}", e)))?;

                let resp = Auth::login(self, ctx, &value).await?;
                let buf = wasmbus_rpc::common::serialize(&resp)?;

                Ok(buf)
            }
            "GetUserRole" => {
                let value: String = wasmbus_rpc::common::deserialize(&message.arg)
                    .map_err(|e| RpcError::Deser(format!("'String': {}", e)))?;

                let resp = Auth::get_user_role(self, ctx, &value).await?;
                let buf = wasmbus_rpc::common::serialize(&resp)?;

                Ok(buf)
            }
            "GetRoles" => {
                let resp = Auth::get_roles(self, ctx).await?;
                let buf = wasmbus_rpc::common::serialize(&resp)?;

                Ok(buf)
            }
            "InitTables" => {
                let resp = Auth::init_tables(self, ctx).await?;
                let buf = wasmbus_rpc::common::serialize(&resp)?;

                Ok(buf)
            }
            _ => Err(RpcError::MethodNotHandled(format!(
                "Auth::{}",
                message.method
            ))),
        }
    }
}

/// AuthSender sends messages to a Auth service
/// Description of Auth service
/// client for sending Auth messages
#[derive(Debug)]
pub struct AuthSender<T: Transport> {
    transport: T,
}

impl<T: Transport> AuthSender<T> {
    /// Constructs a AuthSender with the specified transport
    pub fn via(transport: T) -> Self {
        Self { transport }
    }

    pub fn set_timeout(&self, interval: std::time::Duration) {
        self.transport.set_timeout(interval);
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl<'send> AuthSender<wasmbus_rpc::provider::ProviderTransport<'send>> {
    /// Constructs a Sender using an actor's LinkDefinition,
    /// Uses the provider's HostBridge for rpc
    pub fn for_actor(ld: &'send wasmbus_rpc::core::LinkDefinition) -> Self {
        Self {
            transport: wasmbus_rpc::provider::ProviderTransport::new(ld, None),
        }
    }
}
#[cfg(target_arch = "wasm32")]
impl AuthSender<wasmbus_rpc::actor::prelude::WasmHost> {
    /// Constructs a client for actor-to-actor messaging
    /// using the recipient actor's public key
    pub fn to_actor(actor_id: &str) -> Self {
        let transport =
            wasmbus_rpc::actor::prelude::WasmHost::to_actor(actor_id.to_string()).unwrap();
        Self { transport }
    }
}
#[async_trait]
impl<T: Transport + std::marker::Sync + std::marker::Send> Auth for AuthSender<T> {
    #[allow(unused)]
    /// Registers a user
    async fn register(&self, ctx: &Context, arg: &User) -> RpcResult<User> {
        let buf = wasmbus_rpc::common::serialize(arg)?;

        let resp = self
            .transport
            .send(
                ctx,
                Message {
                    method: "Auth.Register",
                    arg: Cow::Borrowed(&buf),
                },
                None,
            )
            .await?;

        let value: User = wasmbus_rpc::common::deserialize(&resp)
            .map_err(|e| RpcError::Deser(format!("'{}': User", e)))?;
        Ok(value)
    }
    #[allow(unused)]
    async fn login(&self, ctx: &Context, arg: &LoginRequest) -> RpcResult<String> {
        let buf = wasmbus_rpc::common::serialize(arg)?;

        let resp = self
            .transport
            .send(
                ctx,
                Message {
                    method: "Auth.Login",
                    arg: Cow::Borrowed(&buf),
                },
                None,
            )
            .await?;

        let value: String = wasmbus_rpc::common::deserialize(&resp)
            .map_err(|e| RpcError::Deser(format!("'{}': String", e)))?;
        Ok(value)
    }
    #[allow(unused)]
    async fn get_user_role<TS: ToString + ?Sized + std::marker::Sync>(
        &self,
        ctx: &Context,
        arg: &TS,
    ) -> RpcResult<Role> {
        let buf = wasmbus_rpc::common::serialize(&arg.to_string())?;

        let resp = self
            .transport
            .send(
                ctx,
                Message {
                    method: "Auth.GetUserRole",
                    arg: Cow::Borrowed(&buf),
                },
                None,
            )
            .await?;

        let value: Role = wasmbus_rpc::common::deserialize(&resp)
            .map_err(|e| RpcError::Deser(format!("'{}': Role", e)))?;
        Ok(value)
    }
    #[allow(unused)]
    async fn get_roles(&self, ctx: &Context) -> RpcResult<Roles> {
        let buf = *b"";
        let resp = self
            .transport
            .send(
                ctx,
                Message {
                    method: "Auth.GetRoles",
                    arg: Cow::Borrowed(&buf),
                },
                None,
            )
            .await?;

        let value: Roles = wasmbus_rpc::common::deserialize(&resp)
            .map_err(|e| RpcError::Deser(format!("'{}': Roles", e)))?;
        Ok(value)
    }
    #[allow(unused)]
    async fn init_tables(&self, ctx: &Context) -> RpcResult<bool> {
        let buf = *b"";
        let resp = self
            .transport
            .send(
                ctx,
                Message {
                    method: "Auth.InitTables",
                    arg: Cow::Borrowed(&buf),
                },
                None,
            )
            .await?;

        let value: bool = wasmbus_rpc::common::deserialize(&resp)
            .map_err(|e| RpcError::Deser(format!("'{}': Boolean", e)))?;
        Ok(value)
    }
}
