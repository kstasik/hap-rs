// this file is auto-generated by hap-codegen

use serde::ser::{Serialize, SerializeStruct, Serializer};

use crate::{
    service::HapService,
    characteristic::{
        HapCharacteristic,
		occupancy_detected::OccupancyDetectedCharacteristic,
		name::NameCharacteristic,
		status_active::StatusActiveCharacteristic,
		status_fault::StatusFaultCharacteristic,
		status_tampered::StatusTamperedCharacteristic,
		status_low_battery::StatusLowBatteryCharacteristic,
	},
    HapType,
};

/// Occupancy Sensor Service.
#[derive(Debug, Default)]
pub struct OccupancySensorService {
    /// Instance ID of the Occupancy Sensor Service.
    id: u64,
    /// `HapType` of the Occupancy Sensor Service.
    hap_type: HapType,
    /// When set to true, this service is not visible to user.
    hidden: bool,
    /// When set to true, this is the primary service on the accessory.
    primary: bool,
    /// An array of numbers containing the instance IDs of the services that this service links to.
    linked_services: Vec<u64>,

	/// Occupancy Detected Characteristic (required).
	pub occupancy_detected: OccupancyDetectedCharacteristic,

	/// Name Characteristic (optional).
	pub name: Option<NameCharacteristic>,
	/// Status Active Characteristic (optional).
	pub status_active: Option<StatusActiveCharacteristic>,
	/// Status Fault Characteristic (optional).
	pub status_fault: Option<StatusFaultCharacteristic>,
	/// Status Tampered Characteristic (optional).
	pub status_tampered: Option<StatusTamperedCharacteristic>,
	/// Status Low Battery Characteristic (optional).
	pub status_low_battery: Option<StatusLowBatteryCharacteristic>,
}

impl OccupancySensorService {
    /// Creates a new Occupancy Sensor Service.
    pub fn new(id: u64, accessory_id: u64) -> Self {
        Self {
            id,
            hap_type: HapType::OccupancySensor,
			occupancy_detected: OccupancyDetectedCharacteristic::new(id + 1 + 0, accessory_id),
			name: Some(NameCharacteristic::new(id + 1 + 0 + 1, accessory_id)),
			status_active: Some(StatusActiveCharacteristic::new(id + 1 + 1 + 1, accessory_id)),
			status_fault: Some(StatusFaultCharacteristic::new(id + 1 + 2 + 1, accessory_id)),
			status_tampered: Some(StatusTamperedCharacteristic::new(id + 1 + 3 + 1, accessory_id)),
			status_low_battery: Some(StatusLowBatteryCharacteristic::new(id + 1 + 4 + 1, accessory_id)),
			..Default::default()
        }
    }
}

impl HapService for OccupancySensorService {
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
			&self.occupancy_detected,
		];
		if let Some(c) = &self.name {
		    characteristics.push(c);
		}
		if let Some(c) = &self.status_active {
		    characteristics.push(c);
		}
		if let Some(c) = &self.status_fault {
		    characteristics.push(c);
		}
		if let Some(c) = &self.status_tampered {
		    characteristics.push(c);
		}
		if let Some(c) = &self.status_low_battery {
		    characteristics.push(c);
		}
		characteristics
    }

    fn get_mut_characteristics(&mut self) -> Vec<&mut dyn HapCharacteristic> {
        let mut characteristics: Vec<&mut dyn HapCharacteristic> = vec![
			&mut self.occupancy_detected,
		];
		if let Some(c) = &mut self.name {
		    characteristics.push(c);
		}
		if let Some(c) = &mut self.status_active {
		    characteristics.push(c);
		}
		if let Some(c) = &mut self.status_fault {
		    characteristics.push(c);
		}
		if let Some(c) = &mut self.status_tampered {
		    characteristics.push(c);
		}
		if let Some(c) = &mut self.status_low_battery {
		    characteristics.push(c);
		}
		characteristics
    }
}

impl Serialize for OccupancySensorService {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut state = serializer.serialize_struct("HapService", 5)?;
        state.serialize_field("iid", &self.get_id())?;
        state.serialize_field("type", &self.get_type())?;
        state.serialize_field("hidden", &self.get_hidden())?;
        state.serialize_field("primary", &self.get_primary())?;
        state.serialize_field("characteristics", &self.get_characteristics())?;
        // linked services left out for now
        state.end()
    }
}
