struct Workflow {
    id: String,
    version: String,
    description: Description,
    parameters: Vec<Param>,
    environments: Vec<Env>,
    pipeline: Vec<Stage>,
}

/// A simple description node
/// ### Example
/// `description "a description of this block"`
pub struct Description {
    value: String,
}

/// A parameter of workflow execution.
/// It can represent a glob or a literal value
/// ### Example
/// ```kdl
/// parameters {
///     param "name" path="file://here/*.zip" sort="name"
/// }
/// ```
enum Param {
    /// A glob or file path
    /// ### Example
    /// ```kdl
    /// param "name" path="file://here/*.zip" sort="name"
    /// ```
    Glob {
        name: String,
        path: String,
        sort: GlobSort,
    },
    /// A param holding a (literal)[LiteralValue], static value.
    /// ### Example
    /// ```kdl
    /// param "name" {
    ///     - 1
    ///     - abc
    /// }
    /// ```
    Literal { name: String, value: LiteralValue },
}
/// A literal value, either a string, a number or a list of literal values
enum LiteralValue {
    String(String),
    Number(f64),
    List(Vec<LiteralValue>),
}

/// Sorting options supported by Glob
enum GlobSort {
    Name,
    Size,
}

/// An execution environment declaration
struct Env {
    /// A reference to this environment
    name: String,
    /// The image definition to use on this environment
    image: Image,
}

/// An image definition
struct Image {
    tag: String,
    digest: String,
    resources: Resources,
}

struct Resources {
    cpu: u8,
    memory: Memory,
}

enum Memory {
    String(String),
    Megabytes(u64),
    Gigabytes(u64),
}

struct Stage {
    name: String,
    description: Description,
    strategy: Option<Strategy>,
    inputs: Vec<Input>,
    outputs: Vec<Output>,
    use_env: UseEnv,
}

enum Strategy {}

enum Input {
    Glob { name: String, path: String },
}

enum Output {
    Glob { name: String, path: String },
}
enum UseEnv {
    EnvRef(String),
    Conditional {
        conditions: Vec<ConditionExp>,
        default: String,
    },
}

enum ConditionExp {
    String(String),
    Number(f64),
    If {
        cond: Box<ConditionExp>,
        then: Box<ConditionExp>,
    },
}