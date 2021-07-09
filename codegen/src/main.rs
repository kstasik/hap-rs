use handlebars::{Context, Handlebars, Helper, Output, RenderContext, RenderError, Renderable};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::{
    collections::HashMap,
    fs::{self, File},
    io::Write,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SystemMetadata {
    #[serde(rename = "Version")]
    pub version: usize,
    #[serde(rename = "SchemaVersion")]
    pub schema_version: usize,
    #[serde(rename = "PlistDictionary")]
    pub plist_dictionary: SystemPlistDictionary,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SystemPlistDictionary {
    #[serde(rename = "Version")]
    pub version: usize,
    #[serde(rename = "SchemaVersion")]
    pub schema_version: usize,
    #[serde(rename = "HomeKit")]
    pub homekit: HomeKit,
    #[serde(rename = "HAP")]
    pub hap: Hap,
    /* #[serde(rename = "Assistant")]
     * pub assistant: Assistant, */
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct HomeKit {
    #[serde(rename = "Categories")]
    pub categories: HashMap<String, HomeKitCategory>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct HomeKitCategory {
    #[serde(rename = "DefaultDescription")]
    pub name: String,
    #[serde(rename = "Identifier")]
    pub number: u8,
    #[serde(rename = "UUID")]
    pub uuid: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Hap {
    #[serde(rename = "Base UUID")]
    pub base_uuid: String,
    #[serde(rename = "Characteristics")]
    pub characteristics: HashMap<String, HapCharacteristic>,
    #[serde(rename = "Services")]
    pub services: HashMap<String, HapService>,
    #[serde(rename = "Properties")]
    pub properties: HashMap<String, HapProperty>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct HapCharacteristic {
    #[serde(rename = "ShortUUID")]
    pub short_uuid: String,
    #[serde(rename = "DefaultDescription")]
    pub name: String,
    #[serde(rename = "Format")]
    pub format: String,
    #[serde(rename = "MinValue")]
    pub min_value: Option<Value>,
    #[serde(rename = "MaxValue")]
    pub max_value: Option<Value>,
    #[serde(rename = "StepValue")]
    pub step_value: Option<Value>,
    #[serde(rename = "MaxLength")]
    pub max_length: Option<Value>,
    #[serde(rename = "Units")]
    pub units: Option<String>,
    #[serde(rename = "Properties")]
    pub properties: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct HapService {
    #[serde(rename = "ShortUUID")]
    pub short_uuid: String,
    #[serde(rename = "DefaultDescription")]
    pub name: String,
    #[serde(rename = "Characteristics")]
    pub characteristics: HapServiceCharacteristicRelation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct HapServiceCharacteristicRelation {
    #[serde(rename = "Required")]
    pub required_characteristics: Vec<String>,
    #[serde(rename = "Optional")]
    pub optional_characteristics: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct HapProperty {
    #[serde(rename = "DefaultDescription")]
    pub name: String,
    #[serde(rename = "Position")]
    pub number: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct RenderMetadata {
    pub categories: HashMap<String, HomeKitCategory>,
    pub sorted_categories: Vec<HomeKitCategory>,
    pub characteristics: HashMap<String, HapCharacteristic>,
    pub sorted_characteristics: Vec<HapCharacteristic>,
    pub services: HashMap<String, HapService>,
    pub sorted_services: Vec<HapService>,
    pub properties: HashMap<String, HapProperty>,
}

impl From<SystemMetadata> for RenderMetadata {
    fn from(v: SystemMetadata) -> Self {
        let m = v.plist_dictionary;

        let mut sorted_categories = m.homekit.categories.iter().map(|(_, v)| v.clone()).collect::<Vec<_>>();
        sorted_categories.sort_by(|a, b| a.number.partial_cmp(&b.number).unwrap());

        let mut sorted_characteristics = m.hap.characteristics.iter().map(|(_, v)| v.clone()).collect::<Vec<_>>();
        sorted_characteristics.sort_by(|a, b| a.name.cmp(&b.name));

        let mut sorted_services = m.hap.services.iter().map(|(_, v)| v.clone()).collect::<Vec<_>>();
        sorted_services.sort_by(|a, b| a.name.cmp(&b.name));

        Self {
            categories: m.homekit.categories,
            sorted_categories,
            characteristics: m.hap.characteristics,
            sorted_characteristics,
            services: m.hap.services,
            sorted_services,
            properties: m.hap.properties,
        }
    }
}

fn if_eq_helper<'reg, 'rc>(
    h: &Helper<'reg, 'rc>,
    r: &'reg Handlebars,
    c: &Context,
    rc: &mut RenderContext<'reg>,
    out: &mut dyn Output,
) -> Result<(), RenderError> {
    let first = h.param(0).unwrap().value();
    let second = h.param(1).unwrap().value();
    let tmpl = if first == second { h.template() } else { h.inverse() };
    match tmpl {
        Some(ref t) => t.render(r, c, rc, out),
        None => Ok(()),
    }
}

fn trim_helper(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> Result<(), RenderError> {
    let param = h.param(0).unwrap().value();
    if let Some(s) = param.as_str() {
        let trim = s.replace(" ", "").replace(".", "_");
        out.write(&trim)?;
    }
    Ok(())
}

fn file_name_helper(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> Result<(), RenderError> {
    let param = h.param(0).unwrap().value();
    if let Some(s) = param.as_str() {
        let name = s.replace(" ", "_").replace(".", "_").to_lowercase();
        out.write(&name)?;
    }
    Ok(())
}

fn type_helper(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> Result<(), RenderError> {
    let param = h.param(0).unwrap().value();
    if let Some(s) = param.as_str() {
        match s {
            "bool" => {
                out.write("bool")?;
            },
            "uint8" => {
                out.write("u8")?;
            },
            "uint16" => {
                out.write("u16")?;
            },
            "uint32" => {
                out.write("u32")?;
            },
            "uint64" => {
                out.write("u64")?;
            },
            "int" => {
                out.write("i32")?;
            },
            "int32" => {
                out.write("i32")?;
            },
            "float" => {
                out.write("f32")?;
            },
            "string" => {
                out.write("String")?;
            },
            "tlv8" => {
                out.write("Vec<u8>")?;
            },
            "data" => {
                out.write("Vec<u8>")?;
            },
            _ => {
                return Err(RenderError::new("Unknown Characteristic format"));
            },
        }
    }
    Ok(())
}

fn format_helper(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> Result<(), RenderError> {
    let param = h.param(0).unwrap().value();
    if let Some(s) = param.as_str() {
        match s {
            "bool" => {
                out.write("Format::Bool")?;
            },
            "uint8" => {
                out.write("Format::UInt8")?;
            },
            "uint16" => {
                out.write("Format::UInt16")?;
            },
            "uint32" => {
                out.write("Format::UInt32")?;
            },
            "uint64" => {
                out.write("Format::UInt64")?;
            },
            "int" => {
                out.write("Format::Int32")?;
            },
            "int32" => {
                out.write("Format::Int32")?;
            },
            "float" => {
                out.write("Format::Float")?;
            },
            "string" => {
                out.write("Format::String")?;
            },
            "tlv8" => {
                out.write("Format::Tlv8")?;
            },
            "data" => {
                out.write("Format::Data")?;
            },
            _ => {
                return Err(RenderError::new("Unknown Characteristic format"));
            },
        }
    }
    Ok(())
}

fn unit_helper(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> Result<(), RenderError> {
    let param = h.param(0).unwrap().value();
    if let Some(s) = param.as_str() {
        match s {
            "percentage" => {
                out.write("Unit::Percentage")?;
            },
            "arcdegrees" => {
                out.write("Unit::ArcDegrees")?;
            },
            "celsius" => {
                out.write("Unit::Celsius")?;
            },
            "lux" => {
                out.write("Unit::Lux")?;
            },
            "seconds" => {
                out.write("Unit::Seconds")?;
            },
            _ => {
                out.write("Unit::Percentage")?; // TODO - do this properly
            },
            /* _ => {
             *     return Err(RenderError::new("Unknown Characteristic unit"));
             * }, */
        }
    }
    Ok(())
}

fn uuid_helper(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> Result<(), RenderError> {
    let param = h.param(0).unwrap().value();
    if let Some(s) = param.as_str() {
        out.write(&shorten_uuid(&s))?;
    }
    Ok(())
}

fn valid_values_helper(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> Result<(), RenderError> {
    let param = h.param(0).unwrap().value().as_object().unwrap();
    let mut output = String::from("vec![\n");
    for (key, val) in param {
        output.push_str(&format!("\t\t\t\t\t{}, // {}\n", key, val));
    }
    output.push_str("\t\t\t\t]");
    out.write(&output)?;
    Ok(())
}

fn perms_helper(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> Result<(), RenderError> {
    let perms = vec![
        (1 << 0, "\n\t\t\t\tPerm::Events,".to_string()),
        (1 << 1, "\n\t\t\t\tPerm::PairedRead,".to_string()),
        (1 << 2, "\n\t\t\t\tPerm::PairedWrite,".to_string()),
        // Relevant for Bluetooth.
        // (1 << 3, "\n\t\t\t\tPerm::Broadcast,".to_string()),
        // aa set by homed just signals that aa may be supported. Setting up aa will always require a custom made app
        // though. (1 << 4, "\n\t\t\t\tPerm::AdditionalAuthorization,".to_string()),
        (1 << 5, "\n\t\t\t\tPerm::TimedWrite,".to_string()),
        (1 << 6, "\n\t\t\t\tPerm::Hidden,".to_string()),
        (1 << 7, "\n\t\t\t\tPerm::WriteResponse,".to_string()),
    ];

    let properties_bitmap = h.param(0).unwrap().value().as_u64().unwrap();

    for (bitmap, name) in perms {
        // if it stays the same, the bit is set
        if (bitmap | properties_bitmap) == properties_bitmap {
            out.write(&name)?;
        }
    }

    Ok(())
}

fn float_helper(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> Result<(), RenderError> {
    let format = h.param(0).unwrap().value().as_str().unwrap();
    if format == "float" {
        out.write(" as f32")?;
    }
    Ok(())
}

fn array_length_helper(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> Result<(), RenderError> {
    let array = h.param(0).unwrap().value().as_array().unwrap();
    out.write(&format!("{}", array.len()))?;
    Ok(())
}

fn shorten_uuid(id: &str) -> String { id.trim_start_matches('0').to_owned() }

fn snake_case_helper(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> Result<(), RenderError> {
    let param = h.param(0).unwrap().value().as_str().unwrap();
    let name = param
        .replace(" ", "_")
        .replace(".", "_")
        .replace("-", "_")
        .to_lowercase();
    out.write(&name)?;
    Ok(())
}

fn pascal_case_helper(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> Result<(), RenderError> {
    let param = h.param(0).unwrap().value().as_str().unwrap().to_owned();
    let param = param.replace("-", " ");
    let name = param
        .to_lowercase()
        .split(" ")
        .into_iter()
        .map(|word| {
            let mut c = word.chars().collect::<Vec<char>>();
            c[0] = c[0].to_uppercase().nth(0).unwrap();
            c.into_iter().collect::<String>()
        })
        .collect::<String>();
    let name = name.replace(" ", "").replace(".", "_");
    out.write(&name)?;
    Ok(())
}

static CATEGORIES: &'static str = "// this file is auto-generated by hap-codegen\n
use serde::{Deserialize, Serialize};

/// HAP Accessory category.
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum AccessoryCategory {
{{#each sorted_categories as |c|}}\
\t{{pascal_case c.DefaultDescription}} = {{c.Identifier}},
{{/each}}\
}
";

static HAP_TYPE: &'static str = "// this file is auto-generated by hap-codegen\n
use serde::{
    de::{self, Deserialize, Deserializer},
    ser::{Serialize, Serializer},
};
use std::str::FromStr;

use crate::Error;

/// HAP Service and Characteristic type representation.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum HapType {
    Unknown,
{{#each sorted_characteristics as |c|}}\
\t{{pascal_case c.DefaultDescription}},
{{/each}}\
{{#each sorted_services as |s|}}\
\t{{pascal_case s.DefaultDescription}},
{{/each}}\
}

impl ToString for HapType {
    fn to_string(&self) -> String {
        match self {
            HapType::Unknown => \"unknown\".into(),
{{#each sorted_characteristics as |c|}}\
\t\t\tHapType::{{pascal_case c.DefaultDescription}} => \"{{uuid c.ShortUUID}}\".into(),
{{/each}}\
{{#each sorted_services as |s|}}\
\t\t\tHapType::{{pascal_case s.DefaultDescription}} => \"{{uuid s.ShortUUID}}\".into(),
{{/each}}\
\t\t}
    }
}

impl FromStr for HapType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            \"unknown\" => Ok(HapType::Unknown),
{{#each sorted_characteristics as |c|}}\
\t\t\t\"{{uuid c.ShortUUID}}\" => Ok(HapType::{{pascal_case c.DefaultDescription}}),
{{/each}}\
{{#each sorted_services as |s|}}\
\t\t\t\"{{uuid s.ShortUUID}}\" => Ok(HapType::{{pascal_case s.DefaultDescription}}),
{{/each}}\
\t\t\t_ => Err(Error::InvalidHapTypeString(s.to_string())),
\t\t}
    }
}

impl Default for HapType {
    fn default() -> HapType { HapType::Unknown }
}

impl<'de> Deserialize<'de> for HapType {
    fn deserialize<D>(deserializer: D) -> Result<HapType, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let hap_type = HapType::from_str(&s).map_err(de::Error::custom)?;
        Ok(hap_type)
    }
}

impl Serialize for HapType {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.to_string())
    }
}
";

static CHARACTERISTIC: &'static str = "// this file is auto-generated by hap-codegen\n
use async_trait::async_trait;
use serde::Serialize;
use serde_json::json;

use crate::{
    characteristic::{
        AsyncCharacteristicCallbacks,
        Characteristic,
        CharacteristicCallbacks,
        Format,
        HapCharacteristic,
        HapCharacteristicSetup,
        HapType,
        OnReadFn,
        OnReadFuture,
        OnUpdateFn,
        OnUpdateFuture,
        Perm,
        Unit,
    },
    pointer,
    Error,
    Result,
};

// TODO - re-check MaximumDataLength & ValidValues
/// {{characteristic.DefaultDescription}} Characteristic.
#[derive(Debug, Default, Serialize)]
pub struct {{pascal_case characteristic.DefaultDescription}}Characteristic(Characteristic<{{type characteristic.Format}}>);

impl {{pascal_case characteristic.DefaultDescription}}Characteristic {
    /// Creates a new {{characteristic.DefaultDescription}} Characteristic.
    pub fn new(id: u64, accessory_id: u64) -> Self {
        Self(Characteristic::<{{type characteristic.Format}}> {
            id,
            accessory_id,
            hap_type: HapType::{{pascal_case characteristic.DefaultDescription}},
            format: {{format characteristic.Format}},
            perms: vec![{{perms characteristic.Properties}}
            ],\
            {{#if characteristic.Units}}\n\t\t\tunit: Some({{unit characteristic.Units}}),{{/if}}\
            {{#if characteristic.MaxValue includeZero=true}}\n\t\t\tmax_value: Some({{characteristic.MaxValue}}{{float characteristic.Format}}),{{/if}}\
            {{#if characteristic.MinValue includeZero=true}}\n\t\t\tmin_value: Some({{characteristic.MinValue}}{{float characteristic.Format}}),{{/if}}\
            {{#if characteristic.StepValue includeZero=true}}\n\t\t\tstep_value: Some({{characteristic.StepValue}}{{float characteristic.Format}}),{{/if}}\
            {{#if characteristic.MaximumLength includeZero=true}}\n\t\t\tmax_len: Some({{characteristic.MaximumLength}}{{float characteristic.Format}}),{{/if}}\
            {{#if characteristic.MaximumDataLength includeZero=true}}\n\t\t\tmax_data_len: Some({{characteristic.MaximumDataLength}}{{float characteristic.Format}}),{{/if}}\
            {{#if characteristic.ValidValues includeZero=true}}\n\t\t\tvalid_values: Some({{valid_values characteristic.ValidValues}}),{{/if}}
            ..Default::default()
        })
    }
}

#[async_trait]
impl HapCharacteristic for {{pascal_case characteristic.DefaultDescription}}Characteristic {
    fn get_id(&self) -> u64 { self.0.get_id() }

    fn get_type(&self) -> HapType { self.0.get_type() }

    fn get_format(&self) -> Format { self.0.get_format() }

    fn get_perms(&self) -> Vec<Perm> { self.0.get_perms() }

    fn get_event_notifications(&self) -> Option<bool> { self.0.get_event_notifications() }

    fn set_event_notifications(&mut self, event_notifications: Option<bool>) {
        self.0.set_event_notifications(event_notifications)
    }

    async fn get_value(&mut self) -> Result<serde_json::Value> {
        let value = self.0.get_value().await?;
        Ok(json!(value))
    }

    async fn set_value(&mut self, value: serde_json::Value) -> Result<()> {
        let v;
        // for whatever reason, the controller is setting boolean values either as a boolean or as an integer
        if self.0.format == Format::Bool && value.is_number() {
            let num_v: u8 = serde_json::from_value(value)?;
            if num_v == 0 {
                v = serde_json::from_value(json!(false))?;
            } else if num_v == 1 {
                v = serde_json::from_value(json!(true))?;
            } else {
                return Err(Error::InvalidValue(self.get_format()));
            }
        } else {
            v = serde_json::from_value(value).map_err(|_| Error::InvalidValue(self.get_format()))?;
        }
        self.0.set_value(v).await
    }

    fn get_unit(&self) -> Option<Unit> { self.0.get_unit() }

    fn get_max_value(&self) -> Option<serde_json::Value> { self.0.get_max_value().map(|v| json!(v)) }

    fn get_min_value(&self) -> Option<serde_json::Value> { self.0.get_min_value().map(|v| json!(v)) }

    fn get_step_value(&self) -> Option<serde_json::Value> { self.0.get_step_value().map(|v| json!(v)) }

    fn get_max_len(&self) -> Option<u16> { self.0.get_max_len() }
}

impl HapCharacteristicSetup for {{pascal_case characteristic.DefaultDescription}}Characteristic {
    fn set_event_emitter(&mut self, event_emitter: Option<pointer::EventEmitter>) {
        self.0.set_event_emitter(event_emitter)
    }
}

impl CharacteristicCallbacks<{{type characteristic.Format}}> for {{pascal_case characteristic.DefaultDescription}}Characteristic {
    fn on_read(&mut self, f: Option<impl OnReadFn<{{type characteristic.Format}}>>) { self.0.on_read(f) }

    fn on_update(&mut self, f: Option<impl OnUpdateFn<{{type characteristic.Format}}>>) { self.0.on_update(f) }
}

impl AsyncCharacteristicCallbacks<{{type characteristic.Format}}> for {{pascal_case characteristic.DefaultDescription}}Characteristic {
    fn on_read_async(&mut self, f: Option<impl OnReadFuture<{{type characteristic.Format}}>>) { self.0.on_read_async(f) }

    fn on_update_async(&mut self, f: Option<impl OnUpdateFuture<{{type characteristic.Format}}>>) { self.0.on_update_async(f) }
}
";

static CHARACTERISTIC_MOD: &'static str = "// this file is auto-generated by hap-codegen
{{#each characteristics as |c|}}\npub mod {{c}};{{/each}}
";

static SERVICE: &'static str = "// this file is auto-generated by hap-codegen\n
use serde::ser::{Serialize, SerializeStruct, Serializer};

use crate::{
    service::HapService,
    characteristic::{
        HapCharacteristic,
{{#each required_characteristics as |r|}}\
\t\t{{snake_case r.Name}}::{{pascal_case r.Name}}Characteristic,
{{/each}}\
{{#each optional_characteristics as |r|}}\
\t\t{{snake_case r.Name}}::{{pascal_case r.Name}}Characteristic,
{{/each}}\
\t},
    HapType,
};

/// {{service.Name}} Service.
#[derive(Debug, Default)]
pub struct {{pascal_case service.Name}}Service {
    /// Instance ID of the {{service.Name}} Service.
    id: u64,
    /// `HapType` of the {{service.Name}} Service.
    hap_type: HapType,
    /// When set to true, this service is not visible to user.
    hidden: bool,
    /// When set to true, this is the primary service on the accessory.
    primary: bool,
    /// An array of numbers containing the instance IDs of the services that this service links to.
    linked_services: Vec<u64>,

{{#each required_characteristics as |r|}}\
\t/// {{r.Name}} Characteristic (required).
\tpub {{snake_case r.Name}}: {{pascal_case r.Name}}Characteristic,
{{/each}}\
\n{{#each optional_characteristics as |o|}}\
\t/// {{o.Name}} Characteristic (optional).
\tpub {{snake_case o.Name}}: Option<{{pascal_case o.Name}}Characteristic>,
{{/each}}\
}

impl {{pascal_case service.Name}}Service {
    /// Creates a new {{service.Name}} Service.
    pub fn new(id: u64, accessory_id: u64) -> Self {
        Self {
            id,
            hap_type: HapType::{{pascal_case service.Name}},
{{#each required_characteristics as |r|}}\
\t\t\t{{snake_case r.Name}}: {{pascal_case r.Name}}Characteristic::new(id + 1 + {{@index}}, accessory_id),
{{/each}}\
{{#each optional_characteristics as |o|}}\
\t\t\t{{snake_case o.Name}}: Some({{pascal_case o.Name}}Characteristic::new(id + 1 + {{@index}} + {{array_length ../required_characteristics}}, accessory_id)),
{{/each}}\
        \t\t\t..Default::default()
        }
    }
}

impl HapService for {{pascal_case service.Name}}Service {
    fn get_id(&self) -> u64 {
        self.id
    }

    fn get_type(&self) -> HapType {
        self.hap_type
    }

    fn get_hidden(&self) -> bool {
        self.hidden
    }

    fn set_hidden(&mut self, hidden: bool) {
        self.hidden = hidden;
    }

    fn get_primary(&self) -> bool {
        self.primary
    }

    fn set_primary(&mut self, primary: bool) {
        self.primary = primary;
    }

    fn get_linked_services(&self) -> Vec<u64> {
        self.linked_services.clone()
    }

    fn set_linked_services(&mut self, linked_services: Vec<u64>) {
        self.linked_services = linked_services;
    }

    fn get_characteristic(&self, hap_type: HapType) -> Option<&dyn HapCharacteristic> {
        for characteristic in self.get_characteristics() {
            if characteristic.get_type() == hap_type {
                return Some(characteristic);
            }
        }
        None
    }

    fn get_mut_characteristic(&mut self, hap_type: HapType) -> Option<&mut dyn HapCharacteristic> {
        for characteristic in self.get_mut_characteristics() {
            if characteristic.get_type() == hap_type {
                return Some(characteristic);
            }
        }
        None
    }

    fn get_characteristics(&self) -> Vec<&dyn HapCharacteristic> {
        let mut characteristics: Vec<&dyn HapCharacteristic> = vec![
{{#each required_characteristics as |r|}}\
\t\t\t&self.{{snake_case r.Name}},
{{/each}}\
        \t\t];
{{#each optional_characteristics as |r|}}\
\t\tif let Some(c) = &self.{{snake_case r.Name}} {
\t\t    characteristics.push(c);
\t\t}
{{/each}}\
        \t\tcharacteristics
    }

    fn get_mut_characteristics(&mut self) -> Vec<&mut dyn HapCharacteristic> {
        let mut characteristics: Vec<&mut dyn HapCharacteristic> = vec![
{{#each required_characteristics as |r|}}\
\t\t\t&mut self.{{snake_case r.Name}},
{{/each}}\
        \t\t];
{{#each optional_characteristics as |r|}}\
\t\tif let Some(c) = &mut self.{{snake_case r.Name}} {
\t\t    characteristics.push(c);
\t\t}
{{/each}}\
        \t\tcharacteristics
    }
}

impl Serialize for {{pascal_case service.Name}}Service {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut state = serializer.serialize_struct(\"HapService\", 5)?;
        state.serialize_field(\"iid\", &self.get_id())?;
        state.serialize_field(\"type\", &self.get_type())?;
        state.serialize_field(\"hidden\", &self.get_hidden())?;
        state.serialize_field(\"primary\", &self.get_primary())?;
        state.serialize_field(\"characteristics\", &self.get_characteristics())?;
        // linked services left out for now
        state.end()
    }
}
";

static SERVICE_MOD: &'static str = "// this file is auto-generated by hap-codegen
{{#each services as |s|}}\npub mod {{s}};{{/each}}
";

static ACCESSORY: &'static str = "// this file is auto-generated by hap-codegen\n
use serde::ser::{Serialize, SerializeStruct, Serializer};

use crate::{
\taccessory::{AccessoryInformation, HapAccessory},
\tservice::{HapService, accessory_information::AccessoryInformationService, {{snake_case service.Name}}::{{pascal_case service.Name}}Service},
\tHapType,
\tResult,
};

/// {{service.Name}} Accessory.
#[derive(Debug, Default)]
pub struct {{pascal_case service.Name}}Accessory {
    /// ID of the {{service.Name}} Accessory.
    id: u64,

    /// Accessory Information Service.
    pub accessory_information: AccessoryInformationService,
    /// {{service.Name}} Service.
    pub {{snake_case service.Name}}: {{pascal_case service.Name}}Service,
}

impl {{pascal_case service.Name}}Accessory {
    /// Creates a new {{service.Name}} Accessory.
    pub fn new(id: u64, information: AccessoryInformation) -> Result<Self> {
        let accessory_information = information.to_service(1, id)?;
        let {{snake_case service.Name}}_id = accessory_information.get_characteristics().len() as u64;
        let mut {{snake_case service.Name}} = {{pascal_case service.Name}}Service::new(1 + {{snake_case service.Name}}_id + 1, id);
        {{snake_case service.Name}}.set_primary(true);

        Ok(Self {
            id,
            accessory_information,
            {{snake_case service.Name}},
        })
    }
}

impl HapAccessory for {{pascal_case service.Name}}Accessory {
    fn get_id(&self) -> u64 {
        self.id
    }

    fn set_id(&mut self, id: u64) {
        self.id = id;
    }

    fn get_service(&self, hap_type: HapType) -> Option<&dyn HapService> {
        for service in self.get_services() {
            if service.get_type() == hap_type {
                return Some(service);
            }
        }
        None
    }

    fn get_mut_service(&mut self, hap_type: HapType) -> Option<&mut dyn HapService> {
        for service in self.get_mut_services() {
            if service.get_type() == hap_type {
                return Some(service);
            }
        }
        None
    }

    fn get_services(&self) -> Vec<&dyn HapService> {
        vec![
            &self.accessory_information,
            &self.{{snake_case service.Name}},
        ]
    }

    fn get_mut_services(&mut self) -> Vec<&mut dyn HapService> {
        vec![
            &mut self.accessory_information,
            &mut self.{{snake_case service.Name}},
        ]
    }
}

impl Serialize for {{pascal_case service.Name}}Accessory {
    fn serialize<S: Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        let mut state = serializer.serialize_struct(\"HapAccessory\", 2)?;
        state.serialize_field(\"aid\", &self.get_id())?;
        state.serialize_field(\"services\", &self.get_services())?;
        state.end()
    }
}
";

static ACCESSORY_MOD: &'static str = "// this file is auto-generated by hap-codegen
{{#each accessories as |a|}}\npub mod {{a}};{{/each}}
";

// static EXAMPLE: &'static str = "\
// use std::net::{IpAddr, SocketAddr};

// use hap::{
//     accessory::{ {{~snake_case service.Name~}} ::{{pascal_case service.Name}}Accessory, AccessoryCategory,
// AccessoryInformation},     server::{IpServer, Server},
//     storage::FileStorage,
//     tokio,
//     Config,
//     MacAddress,
//     Pin,
// };

// #[tokio::main]
// async fn main() {
//     let current_ipv4 = || -> Option<IpAddr> {
//         for iface in pnet::datalink::interfaces() {
//             for ip_network in iface.ips {
//                 if ip_network.is_ipv4() {
//                     let ip = ip_network.ip();
//                     if !ip.is_loopback() {
//                         return Some(ip);
//                     }
//                 }
//             }
//         }
//         None
//     };

//     let lightbulb = {{pascal_case service.Name}}Accessory::new(1, AccessoryInformation {
//         name: \"Acme {{service.Name}}\".into(),
//         ..Default::default()
//     })
//     .unwrap();

//     let config = Config {
//         socket_addr: SocketAddr::new(current_ipv4().unwrap(), 32000),
//         pin: Pin::new([1, 1, 1, 2, 2, 3, 3, 3])?,
//         name: \"Acme {{service.Name}}\".into(),
//         device_id: MacAddress::new([10, 20, 30, 40, 50, 60]),
//         category: AccessoryCategory::{{pascal_case service.Name}},
//         ..Default::default()
//     };
//     let storage = FileStorage::current_dir().await.unwrap();

//     let mut server = IpServer::new(config, storage).unwrap();
//     server.add_accessory(lightbulb).await.unwrap();

//     let handle = server.run_handle();

//     std::env::set_var(\"RUST_LOG\", \"hap=info\");
//     env_logger::init();

//     handle.await;
// }
// ";

fn main() {
    // let simulator_metadata_file = File::open("codegen/gen/simulator.json").unwrap();
    let metadata_file = File::open("codegen/gen/system.json").unwrap();

    // let simulator_metadata: Metadata = serde_json::from_reader(&simulator_metadata_file).unwrap();
    let metadata: SystemMetadata = serde_json::from_reader(&metadata_file).unwrap();
    let metadata = RenderMetadata::from(metadata);

    // let mut metadata = Metadata {
    //     categories: simulator_metadata.categories,
    //     characteristics: simulator_metadata.characteristics,
    //     services: simulator_metadata.services,
    // };

    // metadata.categories.append(&mut unofficial_metadata.categories);
    // metadata
    //     .characteristics
    //     .append(&mut unofficial_metadata.characteristics);
    // metadata.services.append(&mut unofficial_metadata.services);

    // metadata
    //     .categories
    //     .sort_by(|a, b| a.number.partial_cmp(&b.number).unwrap());
    // metadata.characteristics.sort_by(|a, b| a.name.cmp(&b.name));
    // metadata.services.sort_by(|a, b| a.name.cmp(&b.name));

    // let mut metadata_ex = MetadataEx {
    //     metadata,
    //     characteristics: std::collections::HashMap::new(),
    // };
    // let metadata = &metadata_ex.metadata;

    // // build characteristic map
    // for c in &metadata.characteristics {
    //     metadata_ex.characteristics.insert(c.id.to_string(), &c);
    // }

    let mut handlebars = Handlebars::new();
    handlebars.register_helper("if_eq", Box::new(if_eq_helper));
    handlebars.register_helper("trim", Box::new(trim_helper));
    handlebars.register_helper("file_name", Box::new(file_name_helper));
    handlebars.register_helper("format", Box::new(format_helper));
    handlebars.register_helper("type", Box::new(type_helper));
    handlebars.register_helper("unit", Box::new(unit_helper));
    handlebars.register_helper("uuid", Box::new(uuid_helper));
    handlebars.register_helper("valid_values", Box::new(valid_values_helper));
    handlebars.register_helper("perms", Box::new(perms_helper));
    handlebars.register_helper("float", Box::new(float_helper));
    handlebars.register_helper("array_length", Box::new(array_length_helper));
    handlebars.register_helper("snake_case", Box::new(snake_case_helper));
    handlebars.register_helper("pascal_case", Box::new(pascal_case_helper));
    handlebars.register_template_string("categories", CATEGORIES).unwrap();
    handlebars.register_template_string("hap_type", HAP_TYPE).unwrap(); // PascalCase camelCase
    handlebars
        .register_template_string("characteristic", CHARACTERISTIC)
        .unwrap();
    handlebars
        .register_template_string("characteristic_mod", CHARACTERISTIC_MOD)
        .unwrap();
    handlebars.register_template_string("service", SERVICE).unwrap();
    handlebars.register_template_string("service_mod", SERVICE_MOD).unwrap();
    handlebars.register_template_string("accessory", ACCESSORY).unwrap();
    handlebars
        .register_template_string("accessory_mod", ACCESSORY_MOD)
        .unwrap();
    // handlebars.register_template_string("example", EXAMPLE).unwrap();

    let categories = handlebars.render("categories", &metadata).unwrap();
    let categories_path = "src/accessory/category.rs".to_owned();
    let mut categories_file = File::create(&categories_path).unwrap();
    categories_file.write_all(categories.as_bytes()).unwrap();

    let hap_type = handlebars.render("hap_type", &metadata).unwrap();
    let hap_type_path = "src/hap_type.rs".to_owned();
    let mut hap_type_file = File::create(&hap_type_path).unwrap();
    hap_type_file.write_all(hap_type.as_bytes()).unwrap();

    let characteristic_base_path = "src/characteristic/generated/";
    if std::path::Path::new(&characteristic_base_path).exists() {
        fs::remove_dir_all(&characteristic_base_path).unwrap();
    }
    fs::create_dir_all(&characteristic_base_path).unwrap();
    let mut characteristic_names = vec![];
    for (_, c) in &metadata.characteristics {
        let characteristic = handlebars
            .render("characteristic", &json!({ "characteristic": c }))
            .unwrap();
        let characteristic_file_name = c
            .name
            .replace(" ", "_")
            .replace(".", "_")
            .replace("-", "_")
            .to_lowercase();
        let mut characteristic_path = String::from(characteristic_base_path);
        characteristic_path.push_str(&characteristic_file_name);
        characteristic_path.push_str(".rs");
        let mut characteristic_file = File::create(&characteristic_path).unwrap();
        characteristic_file.write_all(characteristic.as_bytes()).unwrap();
        characteristic_names.push(characteristic_file_name);
    }
    characteristic_names.sort();
    let characteristic_mod = handlebars
        .render(
            "characteristic_mod",
            &json!({ "characteristics": characteristic_names }),
        )
        .unwrap();
    let mut characteristic_mod_file = File::create(&format!("{}mod.rs", characteristic_base_path)).unwrap();
    characteristic_mod_file
        .write_all(characteristic_mod.as_bytes())
        .unwrap();

    // let service_base_path = "src/service/generated/";
    // let accessory_base_path = "src/accessory/generated/";
    // if std::path::Path::new(&service_base_path).exists() {
    //     fs::remove_dir_all(&service_base_path).unwrap();
    // }
    // if std::path::Path::new(&accessory_base_path).exists() {
    //     fs::remove_dir_all(&accessory_base_path).unwrap();
    // }
    // fs::create_dir_all(&service_base_path).unwrap();
    // fs::create_dir_all(&accessory_base_path).unwrap();
    // let mut service_names = vec![];
    // let mut accessory_names = vec![];
    // for s in &metadata.services {
    //     let mut required_characteristics = Vec::new();
    //     let mut optional_characteristics = Vec::new();

    //     for c in &s.required_characteristics {
    //         required_characteristics.push(metadata_ex.characteristics[c]);
    //     }

    //     for c in &s.optional_characteristics {
    //         optional_characteristics.push(metadata_ex.characteristics[c]);
    //     }

    //     let service = handlebars
    //         .render(
    //             "service",
    //             &json!({
    //                 "service": s,
    //                 "required_characteristics": &required_characteristics,
    //                 "optional_characteristics": &optional_characteristics,
    //             }),
    //         )
    //         .unwrap();

    //     let service_file_name = s.name.replace(" ", "_").replace(".", "_").to_lowercase();
    //     let mut service_path = String::from(service_base_path);
    //     service_path.push_str(&service_file_name);
    //     service_path.push_str(".rs");
    //     let mut service_file = File::create(&service_path).unwrap();
    //     service_file.write_all(service.as_bytes()).unwrap();
    //     service_names.push(service_file_name.clone());

    //     if s.name != "Accessory Information"
    //         && s.name != "Battery Service"
    //         && s.name != "Camera RTP Stream Management"
    //         && s.name != "Doorbell"
    //         && s.name != "Faucet"
    //         && s.name != "Filter Maintenance"
    //         && s.name != "Irrigation System"
    //         && s.name != "Lock Management"
    //         && s.name != "Lock Mechanism"
    //         && s.name != "Microphone"
    //         && s.name != "Service Label"
    //         && s.name != "Slat"
    //         && s.name != "Speaker"
    //         && s.name != "Television"
    //         && s.name != "Input Source"
    //     {
    //         let accessory = handlebars
    //             .render(
    //                 "accessory",
    //                 &json!({"service": s, "characteristics": &metadata.characteristics}),
    //             )
    //             .unwrap();
    //         let mut accessory_path = String::from(accessory_base_path);
    //         accessory_path.push_str(&service_file_name);
    //         accessory_path.push_str(".rs");
    //         let mut accessory_file = File::create(&accessory_path).unwrap();
    //         accessory_file.write_all(accessory.as_bytes()).unwrap();
    //         accessory_names.push(service_file_name);
    //     }
    // }
    // let service_mod = handlebars
    //     .render("service_mod", &json!({ "services": service_names }))
    //     .unwrap();
    // let mut service_mod_file = File::create(&format!("{}mod.rs", service_base_path)).unwrap();
    // service_mod_file.write_all(service_mod.as_bytes()).unwrap();
    // let accessory_mod = handlebars
    //     .render("accessory_mod", &json!({ "accessories": accessory_names }))
    //     .unwrap();
    // let mut accessory_mod_file = File::create(&format!("{}mod.rs", accessory_base_path)).unwrap();
    // accessory_mod_file.write_all(accessory_mod.as_bytes()).unwrap();
}
