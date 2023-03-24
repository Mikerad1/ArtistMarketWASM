// auth.smithy
//

// Tell the code generator how to reference symbols defined in this namespace
metadata package = [ { namespace: "za.co.mfrtechnologies.wasminterfaces.auth", crate: "thehaven_interfaces" } ]

namespace za.co.mfrtechnologies.wasminterfaces.auth

use org.wasmcloud.model#wasmbus

/// Description of Auth service
@wasmbus( actorReceive: true )
service Auth {
  version: "0.1",
  operations: [ Register, Login, GetUserRole ]
}

///Registers a user
operation Register {
  input: User,
  output: User
}

//Login a user
operation Login {
  input: LoginRequest,
  output: String
}

operation GetUserRole {
    input: String
    output: Role
}


structure User {
  @required
  id: String,
  @required
  username: String,
  @required
  email: String,
  @required
  password: String,
  @required
  createdAt: Timestamp,
  @required
  updatedAt: Timestamp,
  @required
  id_number: String,
  @required
  firstName: String,
  @required
  lastName: String
  @required
  phoneNumber: String,
  @required
  address: String,
  @required
  gender: String,
  @required
  role_id: String
}

structure LoginRequest {
  @required
  username: String,
  @required
  password: String
}

structure Role {
  @required
  id: String,
  @required
  name: String
}