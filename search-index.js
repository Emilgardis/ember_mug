var searchIndex = JSON.parse('{\
"ember_mug":{"doc":"ember_mug | Rust crate for controlling and retrieving data …","t":"NNNNNENNNNHRRCNNNENNNNNNNNNNNENENNNNNNENLLLLLLLLLLLLLLLLLLLLLLLACLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLALLLLLLLLLLLLLLLLLLLLLLLLLLLLLFFFNDNNNNDNDNNNNDDEDNDENNNNNNDEDNNMLLLLLLLLLMMLLLLLLLLLLLLLLLLLLLLLLLLMLLLLLLLLLLLLLLLMLLLLLLLLLLLLLLLLLLLLLLLLLLLMLLLLLLLLLLLLLLLLLMLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLMLMMMLLLLLLLLLLLLLLMLLLLLLLLLLLLLLLMMLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLMLMLLLLLLL","n":["Battery","BtleConnectError","BtleError","BtleError","BtleError","ConnectError","ControlRegisterAddress","ControlRegisterData","CurrentTemp","Dsk","EMBER_ASSIGNED_NUMBER","EMBER_MUG_PUBLIC_SERVICES","EMBER_MUG_SERVICE_UUID","EmberMug","FromUtf8Error","InvalidFormat","JoinError","KnownCharacteristic","LastLocation","LiquidLevel","LiquidState","MugColor","MugId","Name","NoDevice","NoSuchCharacteristic","NoSuchCharacteristic","Ota","PushEvents","ReadError","ReadError","SearchError","SearchError","Statistics","TargetTemp","TemperatureUnit","TimeDateZone","Udsk","WriteError","WriteError","all","as_any","as_any","as_any","as_any","as_any_mut","as_any_mut","as_any_mut","as_any_mut","as_box_any","as_box_any","as_box_any","as_box_any","borrow","borrow","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","btle","btleplug","clone","clone_into","eq","fmt","fmt","fmt","fmt","fmt","fmt","fmt","fmt","fmt","from","from","from","from","from","from","from","from","from","from","from","from","from","from","get","into","into","into","into","into","mug","new","provide","provide","provide","provide","source","source","source","source","to_owned","to_string","to_string","to_string","to_string","try_from","try_from","try_from","try_from","try_from","try_into","try_into","try_into","try_into","try_into","type_id","type_id","type_id","type_id","type_id","get_mugs","get_mugs_on_adapters","search_adapter_for_ember","AuthInfoNotFound","Battery","BatteryVoltageState","Celsius","Charging","ColdNoTempControl","Color","Cooling","EmberMug","Empty","Fahrenheit","Filling","Heating","LastLocation","LiquidLevel","LiquidState","MugMeta","NotCharging","Ota","PushEvent","RefreshBatteryLevel","RefreshDrinkTemperature","RefreshLiquidLevel","RefreshLiquidState","RefreshTargetTemperature","TargetTemperature","Temperature","TemperatureUnit","TimeDateZone","Unknown","WarmNoTempControl","a","as_any","as_any","as_any","as_any_mut","as_any_mut","as_any_mut","as_box_any","as_box_any","as_box_any","b","battery","borrow","borrow","borrow","borrow","borrow","borrow","borrow","borrow","borrow","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","charge","clone","clone_into","command","connect_mug","deserialize","deserialize","deserialize","deserialize","deserialize","deserialize","deserialize","deserialize","deserialize","disconnected","find_and_connect","firmware_version","fmt","fmt","fmt","fmt","fmt","fmt","fmt","fmt","fmt","fmt","fmt","fmt","fmt","fmt","from","from","from","from","from","from","from","from","from","from","from","from","from_degree","g","get_battery","get_characteristic","get_characteristic_on_service","get_characteristics","get_current_temperature","get_dsk","get_liquid_level","get_liquid_state","get_mug_color","get_mug_meta","get_name","get_ota","get_push_event","get_target_temperature","get_temperature_unit","get_time_date_zone","get_udsk","hardware_version","has_liquid","into","into","into","into","into","into","into","into","into","into","into","into","is_auth_info_not_found","is_battery_voltage_state","is_charging","is_cold_no_temp_control","is_connected","is_cooling","is_empty","is_empty","is_filling","is_heating","is_not_charging","is_refresh_battery_level","is_refresh_drink_temperature","is_refresh_liquid_level","is_refresh_liquid_state","is_refresh_target_temperature","is_target_temperature","is_unknown","is_warm_no_temp_control","level","listen_push_events","mug_id","offset","r","read","read_deserialize","read_options","read_options","read_options","read_options","read_options","read_options","read_options","read_options","read_options","read_options","read_options","request","serial_number","serialize","serialize","serialize","serialize","serialize","serialize","serialize","serialize","serialize","set_mug_color","set_name","set_target_temperature","set_temperature_unit","set_time_date_zone","subscribe_push_events","temperature","temperature","to_degree","to_owned","to_string","to_string","to_string","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_into","try_into","try_into","try_into","try_into","try_into","try_into","try_into","try_into","try_into","try_into","try_into","type_id","type_id","type_id","type_id","type_id","type_id","type_id","type_id","type_id","type_id","type_id","type_id","unix_timestamp","unsubscribe_push_events","volt","write","write_options","write_options","write_options","write_options","write_options","write_options"],"q":[[0,"ember_mug"],[127,"ember_mug::btle"],[130,"ember_mug::mug"]],"d":["The current battery level of the mug.","Couldn’t connect device","Search operation failed","Read from BLE failed","Write with BLE failed","Errors when trying to connect to an Ember Mug","Temperature lock address","Temperature lock data","The current temperature of the mug’s contents.","Device Secret Key","Assigned Bluetooth company identifier for …","Known public UUIDs of ember mugs","The UUID for the Ember Mug service","","Failed to convert string to valid UTF-8","Data to be written was invalid","Task join failed","All known characteristics of an Ember Mug","The last known location of the mug.","The current liquid level of the mug.","The state of the liquid in the mug (e.g. solid, liquid, …","The color of the mug.","The mug’s identifier.","The name of the mug.","No device found","Characteristic is missing / not present on device","Characteristic is missing / not present on device","Firmware and hardware information (versions) for the mug.","Events that are sent from the mug","Errors when reading data from an Ember Mug","Reading of data failed","Errors when searching for a device","Search failed","Statistics","The target temperature for the mug’s contents.","The unit of temperature measurement used for the mug.","The current time, date, and time zone of the mug.","(U?) Device Secret Key","Errors when writing data to an Ember Mug","Interpreting source data into bytes failed","Get all known characteristics","","","","","","","","","","","","","","","","","","","","","","","Functions for communicating with BLE to connect to Ember …","","","","","","","","","","","","","","Returns the argument unchanged.","","","Returns the argument unchanged.","","","Returns the argument unchanged.","","","","Returns the argument unchanged.","","Returns the argument unchanged.","","Get the UUID for this characteristic","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Hosts <code>EmberMug</code> and related functions","Create a new known characteristic from UUID","","","","","","","","","","","","","","","","","","","","","","","","","","","","","Get mugs on all adapters","Search for mugs on all adapters","Search given adapter for a mug","Auth info missing","Battery information","Battery voltage state changed","Celcius","Charging","The mug is cold and temperature control is disabled","Color tint for an Ember Mug","The mug is cooling down to the target temperature","An Ember Mug device","The mug is empty","Fahrenheit","The mug is filling with liquid","The mug is heating up to the target temperature","Location","Level of the liquid","Represents the current state of the liquid in an Ember Mug","Metadata for the device","Not Charging","Version information for the device","Events to trigger updates in application state","Refresh battery level","Refresh drink temperature","Refresh liquid level","Refresh liquid state","Refresh target temperature","The mug’s liquid is at the target temperature","Temperature in a certain unit","Temperature unit/scale","Time and date + timezone","The liquid state is unknown","The mug is warm and temperature control is disabled","Alpha value (0-255)","","","","","","","","","","Blue value (0-255)","Battery percentage (5 - 100. Not scaled to 0 - 255)","","","","","","","","","","","","","","","","","","","","","","","","","Charging status. 1 for plugged in, 0 for unplugged","","","Send command to given characteristic on <code>uuid</code>","Connect to specific Ember Mug","","","","","","","","","","Returns when the device is disconnected.","Find and connect to the first available Ember Mug","Firmware version","","","","","","","","","","","","","","","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Convert given degree to a temperature","Green value (0-255)","Retrieves the battery percentage of the mug and other …","Get characteristic on <code>EMBER_MUG_SERVICE_UUID</code> with given …","Get characteristic on given service UUID with given UUID","Get all characteristics","Retrieves the current temperature of the mug","Retrieves the dsk of the cup","Retrieves the level of liquid present in the cup","The current state of the mug","Retrieves the color of the mug’s LED indicator.","Retrieves id of the mug","Retreives the name of the mug.","Info about the current firmware running on the mug.","Events sent by the mug for the application to register to.","Retrieves the target temperature of the mug","Retrieve the current unit of temperature used by the mug.","Get the current date and timezone on the mug","Retrieves the dsk of the cup","Hardware version","Mug has liquid","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Returns <code>true</code> if the push event is <code>AuthInfoNotFound</code>.","Returns <code>true</code> if the push event is <code>BatteryVoltageState</code>.","Returns <code>true</code> if the push event is <code>Charging</code>.","Returns <code>true</code> if the liquid state is <code>ColdNoTempControl</code>.","Returns true if the device is connected, the device might …","Returns <code>true</code> if the liquid state is <code>Cooling</code>.","Mug is empty","Returns <code>true</code> if the liquid state is <code>Empty</code>.","Returns <code>true</code> if the liquid state is <code>Filling</code>.","Returns <code>true</code> if the liquid state is <code>Heating</code>.","Returns <code>true</code> if the push event is <code>NotCharging</code>.","Returns <code>true</code> if the push event is <code>RefreshBatteryLevel</code>.","Returns <code>true</code> if the push event is <code>RefreshDrinkTemperature</code>.","Returns <code>true</code> if the push event is <code>RefreshLiquidLevel</code>.","Returns <code>true</code> if the push event is <code>RefreshLiquidState</code>.","Returns <code>true</code> if the push event is <code>RefreshTargetTemperature</code>.","Returns <code>true</code> if the liquid state is <code>TargetTemperature</code>.","Returns <code>true</code> if the liquid state is <code>Unknown</code>.","Returns <code>true</code> if the liquid state is <code>WarmNoTempControl</code>.","The given amount of liquid","Get a stream of events sent by the mug. You need to use …","ID","Timezone offset (ex: GMT+03)","Red value (0-255)","Deserialize data on given characteristic with <code>uuid</code>","Read data from given characteristic with <code>uuid</code>","","","","","","","","","","","","Send request to given characteristic on <code>uuid</code>","Serial number","","","","","","","","","","Sets the color of the mug’s LED indicator.","Sets the name of the mug.","Set the target temperature of the mug","Set the current unit of temperature used by the mug.","A sink for the mug to store the current date and timezone","Subscribe to events sent by the mug","Battery temperature as UINT16 Little Endian, encoded like …","The temperature in integer value, use …","Convert value to degree","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","Unix timestamp recorded by the app.","Unsubscribe to events sent by the mug","(Legacy) Most likely battery voltage","Write data to given characteristic on <code>uuid</code>","","","","","",""],"i":[4,8,9,10,11,0,4,4,4,4,0,0,0,0,10,11,9,0,4,4,4,4,4,4,8,10,11,4,4,0,10,0,8,4,4,4,4,4,0,11,4,8,9,10,11,8,9,10,11,8,9,10,11,4,8,9,10,11,4,8,9,10,11,0,0,4,4,4,4,8,8,9,9,10,10,11,11,4,8,8,8,9,9,9,10,10,10,10,11,11,11,4,4,8,9,10,11,0,4,8,9,10,11,8,9,10,11,4,8,9,10,11,4,8,9,10,11,4,8,9,10,11,4,8,9,10,11,0,0,0,37,0,37,38,37,34,0,34,0,34,38,34,34,0,0,0,0,37,0,0,37,37,37,37,37,34,0,0,0,34,34,35,34,38,40,34,38,40,34,38,40,35,32,32,41,33,34,35,36,42,37,38,39,26,40,32,41,33,34,35,36,42,37,38,39,26,40,32,26,26,26,26,32,33,34,35,36,37,38,39,40,26,26,42,32,41,33,34,34,35,36,42,37,38,38,39,40,40,32,41,33,34,35,36,42,37,38,39,26,40,40,35,26,26,26,26,26,26,26,26,26,26,26,26,26,26,26,26,26,42,33,32,41,33,34,35,36,42,37,38,39,26,40,37,37,37,34,26,34,33,34,34,34,37,37,37,37,37,37,34,34,34,33,26,36,39,35,26,26,32,41,33,34,35,36,42,37,38,39,40,26,36,32,33,34,35,36,37,38,39,40,26,26,26,26,26,26,32,40,40,26,34,38,40,32,41,33,34,35,36,42,37,38,39,26,40,32,41,33,34,35,36,42,37,38,39,26,40,32,41,33,34,35,36,42,37,38,39,26,40,39,26,32,26,41,35,42,38,39,40],"f":[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,[[]],[[],1],[[],1],[[],1],[[],1],[[],1],[[],1],[[],1],[[],1],[[[3,[2]]],[[3,[1,2]]]],[[[3,[2]]],[[3,[1,2]]]],[[[3,[2]]],[[3,[1,2]]]],[[[3,[2]]],[[3,[1,2]]]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],0,0,[4,4],[[]],[[4,4],5],[[4,6],7],[[8,6],7],[[8,6],7],[[9,6],7],[[9,6],7],[[10,6],7],[[10,6],7],[[11,6],7],[[11,6],7],[[]],[12,8],[9,8],[[]],[12,9],[13,9],[[]],[14,10],[15,10],[12,10],[[]],[12,11],[[]],[14,11],[4,16],[[]],[[]],[[]],[[]],[[]],0,[16,[[17,[4]]]],[18],[18],[18],[18],[8,[[17,[19]]]],[9,[[17,[19]]]],[10,[[17,[19]]]],[11,[[17,[19]]]],[[]],[[],20],[[],20],[[],20],[[],20],[[],21],[[],21],[[],21],[[],21],[[],21],[[],21],[[],21],[[],21],[[],21],[[],21],[[],22],[[],22],[[],22],[[],22],[[],22],[[],[[21,[23,9]]]],[[],23],[[24,[17,[25]]],[[21,[23,12]]]],0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,[[],1],[[],1],[[],1],[[],1],[[],1],[[],1],[[[3,[2]]],[[3,[1,2]]]],[[[3,[2]]],[[3,[1,2]]]],[[[3,[2]]],[[3,[1,2]]]],0,0,[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],0,[26,26],[[]],[[26,4,[0,[27,28,29,30]]],[[21,[11]]]],[24,[[21,[26,8]]]],[31,[[21,[32]]]],[31,[[21,[33]]]],[31,[[21,[34]]]],[31,[[21,[35]]]],[31,[[21,[36]]]],[31,[[21,[37]]]],[31,[[21,[38]]]],[31,[[21,[39]]]],[31,[[21,[40]]]],[26,[[21,[12]]]],[[],[[21,[26,8]]]],0,[[32,6],7],[[41,6],7],[[33,6],7],[[34,6],7],[[34,6],7],[[35,6],7],[[36,6],7],[[42,6],7],[[37,6],7],[[38,6],7],[[38,6],7],[[39,6],7],[[40,6],7],[[40,6],7],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[43,40],0,[26,[[21,[32,10]]]],[[26,16],[[17,[44]]]],[[26,16,16],[[17,[44]]]],[26,45],[26,[[21,[40,10]]]],[26,[[21,[[47,[46]],10]]]],[26,[[21,[33,10]]]],[26,[[21,[34,10]]]],[26,[[21,[35,10]]]],[26,[[21,[36,10]]]],[26,[[21,[20,10]]]],[26,[[21,[42,10]]]],[26,[[21,[37,10]]]],[26,[[21,[40,10]]]],[26,[[21,[38,10]]]],[26,[[21,[39,10]]]],[26,[[21,[[47,[46]],10]]]],0,[33,5],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[37,5],[37,5],[37,5],[34,5],[26,[[21,[5,12]]]],[34,5],[33,5],[34,5],[34,5],[34,5],[37,5],[37,5],[37,5],[37,5],[37,5],[37,5],[34,5],[34,5],[34,5],0,[26,[[21,[[0,[23,29]],10]]]],0,0,0,[[26,4],[[21,[[47,[46]],10]]]],[[26,4],[[21,[[0,[48,49]],10]]]],[[[0,[50,51]],52],[[53,[32]]]],[[[0,[50,51]],52],[[53,[41]]]],[[[0,[50,51]],52],[[53,[33]]]],[[[0,[50,51]],52],[[53,[34]]]],[[[0,[50,51]],52],[[53,[35]]]],[[[0,[50,51]],52],[[53,[36]]]],[[[0,[50,51]],52],[[53,[42]]]],[[[0,[50,51]],52],[[53,[37]]]],[[[0,[50,51]],52],[[53,[38]]]],[[[0,[50,51]],52],[[53,[39]]]],[[[0,[50,51]],52],[[53,[40]]]],[[26,4,[0,[27,28,29,30]]],[[21,[11]]]],0,[[32,54],21],[[33,54],21],[[34,54],21],[[35,54],21],[[36,54],21],[[37,54],21],[[38,54],21],[[39,54],21],[[40,54],21],[[26,35],[[21,[11]]]],[[26,55],[[21,[11]]]],[[26,40],[[21,[11]]]],[[26,38],[[21,[11]]]],[[26,39],[[21,[11]]]],[26,[[21,[10]]]],0,0,[40,43],[[]],[[],20],[[],20],[[],20],[[],21],[[],21],[[],21],[[],21],[[],21],[[],21],[[],21],[[],21],[[],21],[[],21],[[],21],[[],21],[[],21],[[],21],[[],21],[[],21],[[],21],[[],21],[[],21],[[],21],[[],21],[[],21],[[],21],[[],21],[[],22],[[],22],[[],22],[[],22],[[],22],[[],22],[[],22],[[],22],[[],22],[[],22],[[],22],[[],22],0,[26,[[21,[10]]]],0,[[26,56,4,[0,[27,28,29,30]]],[[21,[11]]]],[[41,[0,[57,51]],58],53],[[35,[0,[57,51]],58],53],[[42,[0,[57,51]],58],53],[[38,[0,[57,51]],58],53],[[39,[0,[57,51]],58],53],[[40,[0,[57,51]],58],53]],"c":[],"p":[[8,"Any"],[3,"Global"],[3,"Box"],[4,"KnownCharacteristic"],[15,"bool"],[3,"Formatter"],[6,"Result"],[4,"ConnectError"],[4,"SearchError"],[4,"ReadError"],[4,"WriteError"],[4,"Error"],[3,"JoinError"],[4,"Error"],[3,"FromUtf8Error"],[3,"Uuid"],[4,"Option"],[3,"Demand"],[8,"Error"],[3,"String"],[4,"Result"],[3,"TypeId"],[8,"Stream"],[3,"Adapter"],[3,"BDAddr"],[3,"EmberMug"],[8,"BinWrite"],[8,"WriteEndian"],[8,"Send"],[8,"Sync"],[8,"Deserializer"],[3,"Battery"],[3,"LiquidLevel"],[4,"LiquidState"],[3,"Color"],[3,"MugMeta"],[4,"PushEvent"],[4,"TemperatureUnit"],[3,"TimeDateZone"],[3,"Temperature"],[3,"LastLocation"],[3,"Ota"],[15,"f32"],[3,"Characteristic"],[8,"Iterator"],[15,"u8"],[3,"Vec"],[8,"BinRead"],[8,"ReadEndian"],[8,"Read"],[8,"Seek"],[3,"ReadOptions"],[6,"BinResult"],[8,"Serializer"],[15,"str"],[4,"WriteType"],[8,"Write"],[3,"WriteOptions"]]}\
}');
if (typeof window !== 'undefined' && window.initSearch) {window.initSearch(searchIndex)};
if (typeof exports !== 'undefined') {exports.searchIndex = searchIndex};
