use yang2::context::{Context, ContextFlags};
use yang2::schema::{
    DataValue, DataValueType, SchemaNodeKind, SchemaPathFormat,
};

static SEARCH_DIR: &str = "./assets/yang/";

fn create_context() -> Context {
    // Initialize context.
    let mut ctx = Context::new(ContextFlags::NO_YANGLIBRARY)
        .expect("Failed to create context");
    ctx.set_searchdir(SEARCH_DIR)
        .expect("Failed to set YANG search directory");

    // Load YANG modules.
    ctx.load_module("ietf-interfaces", None, &["pre-provisioning"])
        .expect("Failed to load module");
    ctx.load_module("iana-if-type", None, &[])
        .expect("Failed to load module");
    ctx.load_module("ietf-routing", None, &[])
        .expect("Failed to load module");

    ctx
}

#[test]
fn schema_feature_value() {
    let ctx = create_context();
    let module = ctx.get_module_latest("ietf-interfaces").unwrap();
    assert_eq!(module.feature_value("pre-provisioning"), Ok(true));
    assert_eq!(module.feature_value("if-mib"), Ok(false));
    assert!(module.feature_value("blabla").is_err());
}

#[test]
fn schema_find_xpath() {
    let ctx = create_context();

    assert_eq!(
        ctx.find_xpath("/ietf-interfaces:interfaces/*")
            .expect("Failed to lookup schema data")
            .map(|dnode| dnode.path(SchemaPathFormat::DATA))
            .collect::<Vec<String>>(),
        vec!["/ietf-interfaces:interfaces/interface"]
    );

    assert_eq!(
        ctx.find_xpath("/ietf-interfaces:interfaces/interface/*")
            .expect("Failed to lookup schema data")
            .map(|dnode| dnode.path(SchemaPathFormat::DATA))
            .collect::<Vec<String>>(),
        vec![
            "/ietf-interfaces:interfaces/interface/name",
            "/ietf-interfaces:interfaces/interface/description",
            "/ietf-interfaces:interfaces/interface/type",
            "/ietf-interfaces:interfaces/interface/enabled",
            "/ietf-interfaces:interfaces/interface/oper-status",
            "/ietf-interfaces:interfaces/interface/last-change",
            "/ietf-interfaces:interfaces/interface/phys-address",
            "/ietf-interfaces:interfaces/interface/higher-layer-if",
            "/ietf-interfaces:interfaces/interface/lower-layer-if",
            "/ietf-interfaces:interfaces/interface/speed",
            "/ietf-interfaces:interfaces/interface/statistics",
        ]
    );
}

#[test]
fn schema_find_path() {
    let ctx = create_context();

    assert!(ctx
        .find_path("/ietf-interfaces:interfaces/interface/*")
        .is_err());
    assert!(ctx
        .find_path("/ietf-interfaces:interfaces/interface")
        .is_ok());
}

#[test]
fn schema_iterator_traverse() {
    let ctx = create_context();

    assert_eq!(
        ctx
            .traverse()
            .map(|snode| snode.path(SchemaPathFormat::DATA))
            .collect::<Vec<String>>(),
        vec![
            "/ietf-yang-schema-mount:schema-mounts",
            "/ietf-yang-schema-mount:schema-mounts/namespace",
            "/ietf-yang-schema-mount:schema-mounts/namespace/prefix",
            "/ietf-yang-schema-mount:schema-mounts/namespace/uri",
            "/ietf-yang-schema-mount:schema-mounts/mount-point",
            "/ietf-yang-schema-mount:schema-mounts/mount-point/module",
            "/ietf-yang-schema-mount:schema-mounts/mount-point/label",
            "/ietf-yang-schema-mount:schema-mounts/mount-point/config",
            "/ietf-yang-schema-mount:schema-mounts/mount-point",
            "/ietf-yang-schema-mount:schema-mounts/mount-point",
            "/ietf-yang-schema-mount:schema-mounts/mount-point/inline",
            "/ietf-yang-schema-mount:schema-mounts/mount-point",
            "/ietf-yang-schema-mount:schema-mounts/mount-point/shared-schema",
            "/ietf-yang-schema-mount:schema-mounts/mount-point/shared-schema/parent-reference",
            "/ietf-interfaces:interfaces",
            "/ietf-interfaces:interfaces/interface",
            "/ietf-interfaces:interfaces/interface/name",
            "/ietf-interfaces:interfaces/interface/description",
            "/ietf-interfaces:interfaces/interface/type",
            "/ietf-interfaces:interfaces/interface/enabled",
            "/ietf-interfaces:interfaces/interface/oper-status",
            "/ietf-interfaces:interfaces/interface/last-change",
            "/ietf-interfaces:interfaces/interface/phys-address",
            "/ietf-interfaces:interfaces/interface/higher-layer-if",
            "/ietf-interfaces:interfaces/interface/lower-layer-if",
            "/ietf-interfaces:interfaces/interface/speed",
            "/ietf-interfaces:interfaces/interface/statistics",
            "/ietf-interfaces:interfaces/interface/statistics/discontinuity-time",
            "/ietf-interfaces:interfaces/interface/statistics/in-octets",
            "/ietf-interfaces:interfaces/interface/statistics/in-unicast-pkts",
            "/ietf-interfaces:interfaces/interface/statistics/in-broadcast-pkts",
            "/ietf-interfaces:interfaces/interface/statistics/in-multicast-pkts",
            "/ietf-interfaces:interfaces/interface/statistics/in-discards",
            "/ietf-interfaces:interfaces/interface/statistics/in-errors",
            "/ietf-interfaces:interfaces/interface/statistics/in-unknown-protos",
            "/ietf-interfaces:interfaces/interface/statistics/out-octets",
            "/ietf-interfaces:interfaces/interface/statistics/out-unicast-pkts",
            "/ietf-interfaces:interfaces/interface/statistics/out-broadcast-pkts",
            "/ietf-interfaces:interfaces/interface/statistics/out-multicast-pkts",
            "/ietf-interfaces:interfaces/interface/statistics/out-discards",
            "/ietf-interfaces:interfaces/interface/statistics/out-errors",
            "/ietf-interfaces:interfaces-state",
            "/ietf-interfaces:interfaces-state/interface",
            "/ietf-interfaces:interfaces-state/interface/name",
            "/ietf-interfaces:interfaces-state/interface/type",
            "/ietf-interfaces:interfaces-state/interface/oper-status",
            "/ietf-interfaces:interfaces-state/interface/last-change",
            "/ietf-interfaces:interfaces-state/interface/phys-address",
            "/ietf-interfaces:interfaces-state/interface/higher-layer-if",
            "/ietf-interfaces:interfaces-state/interface/lower-layer-if",
            "/ietf-interfaces:interfaces-state/interface/speed",
            "/ietf-interfaces:interfaces-state/interface/statistics",
            "/ietf-interfaces:interfaces-state/interface/statistics/discontinuity-time",
            "/ietf-interfaces:interfaces-state/interface/statistics/in-octets",
            "/ietf-interfaces:interfaces-state/interface/statistics/in-unicast-pkts",
            "/ietf-interfaces:interfaces-state/interface/statistics/in-broadcast-pkts",
            "/ietf-interfaces:interfaces-state/interface/statistics/in-multicast-pkts",
            "/ietf-interfaces:interfaces-state/interface/statistics/in-discards",
            "/ietf-interfaces:interfaces-state/interface/statistics/in-errors",
            "/ietf-interfaces:interfaces-state/interface/statistics/in-unknown-protos",
            "/ietf-interfaces:interfaces-state/interface/statistics/out-octets",
            "/ietf-interfaces:interfaces-state/interface/statistics/out-unicast-pkts",
            "/ietf-interfaces:interfaces-state/interface/statistics/out-broadcast-pkts",
            "/ietf-interfaces:interfaces-state/interface/statistics/out-multicast-pkts",
            "/ietf-interfaces:interfaces-state/interface/statistics/out-discards",
            "/ietf-interfaces:interfaces-state/interface/statistics/out-errors"
        ]
    );
}

#[test]
fn schema_iterator_ancestors() {
    let ctx = create_context();

    assert_eq!(
        ctx
            .find_path("/ietf-interfaces:interfaces/interface/statistics/discontinuity-time")
            .expect("Failed to lookup schema data")
            .ancestors()
            .map(|snode| snode.path(SchemaPathFormat::DATA))
            .collect::<Vec<String>>(),
        vec![
            "/ietf-interfaces:interfaces/interface/statistics",
            "/ietf-interfaces:interfaces/interface",
            "/ietf-interfaces:interfaces",
        ]
    );
    assert_eq!(
        ctx
            .find_path("/ietf-interfaces:interfaces/interface/statistics/discontinuity-time")
            .expect("Failed to lookup schema data")
            .inclusive_ancestors()
            .map(|snode| snode.path(SchemaPathFormat::DATA))
            .collect::<Vec<String>>(),
        vec![
            "/ietf-interfaces:interfaces/interface/statistics/discontinuity-time",
            "/ietf-interfaces:interfaces/interface/statistics",
            "/ietf-interfaces:interfaces/interface",
            "/ietf-interfaces:interfaces",
        ]
    );
}

#[test]
fn schema_iterator_siblings() {
    let ctx = create_context();

    assert_eq!(
        ctx.find_path("/ietf-interfaces:interfaces/interface/name")
            .expect("Failed to lookup schema data")
            .siblings()
            .map(|snode| snode.path(SchemaPathFormat::DATA))
            .collect::<Vec<String>>(),
        vec![
            "/ietf-interfaces:interfaces/interface/description",
            "/ietf-interfaces:interfaces/interface/type",
            "/ietf-interfaces:interfaces/interface/enabled",
            "/ietf-interfaces:interfaces/interface/oper-status",
            "/ietf-interfaces:interfaces/interface/last-change",
            "/ietf-interfaces:interfaces/interface/phys-address",
            "/ietf-interfaces:interfaces/interface/higher-layer-if",
            "/ietf-interfaces:interfaces/interface/lower-layer-if",
            "/ietf-interfaces:interfaces/interface/speed",
            "/ietf-interfaces:interfaces/interface/statistics",
        ]
    );
    assert_eq!(
        ctx.find_path("/ietf-interfaces:interfaces/interface/name")
            .expect("Failed to lookup schema data")
            .inclusive_siblings()
            .map(|snode| snode.path(SchemaPathFormat::DATA))
            .collect::<Vec<String>>(),
        vec![
            "/ietf-interfaces:interfaces/interface/name",
            "/ietf-interfaces:interfaces/interface/description",
            "/ietf-interfaces:interfaces/interface/type",
            "/ietf-interfaces:interfaces/interface/enabled",
            "/ietf-interfaces:interfaces/interface/oper-status",
            "/ietf-interfaces:interfaces/interface/last-change",
            "/ietf-interfaces:interfaces/interface/phys-address",
            "/ietf-interfaces:interfaces/interface/higher-layer-if",
            "/ietf-interfaces:interfaces/interface/lower-layer-if",
            "/ietf-interfaces:interfaces/interface/speed",
            "/ietf-interfaces:interfaces/interface/statistics",
        ]
    );
}

#[test]
fn schema_iterator_children() {
    let ctx = create_context();

    assert_eq!(
        ctx
            .find_path("/ietf-interfaces:interfaces/interface/statistics")
            .expect("Failed to lookup schema data")
            .children()
            .map(|snode| snode.path(SchemaPathFormat::DATA))
            .collect::<Vec<String>>(),
        vec![
            "/ietf-interfaces:interfaces/interface/statistics/discontinuity-time",
            "/ietf-interfaces:interfaces/interface/statistics/in-octets",
            "/ietf-interfaces:interfaces/interface/statistics/in-unicast-pkts",
            "/ietf-interfaces:interfaces/interface/statistics/in-broadcast-pkts",
            "/ietf-interfaces:interfaces/interface/statistics/in-multicast-pkts",
            "/ietf-interfaces:interfaces/interface/statistics/in-discards",
            "/ietf-interfaces:interfaces/interface/statistics/in-errors",
            "/ietf-interfaces:interfaces/interface/statistics/in-unknown-protos",
            "/ietf-interfaces:interfaces/interface/statistics/out-octets",
            "/ietf-interfaces:interfaces/interface/statistics/out-unicast-pkts",
            "/ietf-interfaces:interfaces/interface/statistics/out-broadcast-pkts",
            "/ietf-interfaces:interfaces/interface/statistics/out-multicast-pkts",
            "/ietf-interfaces:interfaces/interface/statistics/out-discards",
            "/ietf-interfaces:interfaces/interface/statistics/out-errors",
        ]
    );

    assert_eq!(
        ctx.find_path("/ietf-routing:routing/ribs/rib")
            .expect("Failed to lookup schema data")
            .children()
            .map(|snode| snode.path(SchemaPathFormat::DATA))
            .collect::<Vec<String>>(),
        vec![
            "/ietf-routing:routing/ribs/rib/name",
            "/ietf-routing:routing/ribs/rib/address-family",
            "/ietf-routing:routing/ribs/rib/routes",
            "/ietf-routing:routing/ribs/rib/description"
        ]
    );

    assert_eq!(
        ctx.find_path("/ietf-routing:routing/ribs/rib")
            .expect("Failed to lookup schema data")
            .all_children()
            .map(|snode| snode.path(SchemaPathFormat::DATA))
            .collect::<Vec<String>>(),
        vec![
            "/ietf-routing:routing/ribs/rib/name",
            "/ietf-routing:routing/ribs/rib/address-family",
            "/ietf-routing:routing/ribs/rib/routes",
            "/ietf-routing:routing/ribs/rib/description",
            "/ietf-routing:routing/ribs/rib/active-route"
        ]
    );
}

#[test]
fn schema_node_attributes() {
    let ctx = create_context();

    let snode = ctx
        .find_path("/ietf-interfaces:interfaces/interface/enabled")
        .expect("Failed to lookup schema node");
    assert_eq!(snode.kind(), SchemaNodeKind::Leaf);
    assert_eq!(snode.name(), "enabled");
    assert!(snode.description().is_some());
    assert!(snode.reference().is_some());
    assert_eq!(snode.is_config(), true);
    assert_eq!(snode.is_state(), false);
    assert_eq!(snode.is_mandatory(), false);
    assert_eq!(snode.default_value_canonical(), Some("true"));
    assert_eq!(snode.default_value(), Some(DataValue::Bool(true)));
    assert_eq!(snode.leaf_type().unwrap().base_type(), DataValueType::Bool);
    assert!(snode.units().is_none());
    assert!(snode.musts().unwrap().count() == 0);
    assert!(snode.whens().count() == 0);
    assert_eq!(snode.is_status_current(), true);
    assert_eq!(snode.is_status_deprecated(), false);
    assert_eq!(snode.is_status_obsolete(), false);

    let snode = ctx
        .find_path("/ietf-interfaces:interfaces/interface")
        .expect("Failed to lookup schema node");
    assert_eq!(snode.kind(), SchemaNodeKind::List);
    assert_eq!(snode.name(), "interface");
    assert!(snode.description().is_some());
    assert!(snode.reference().is_none());
    assert_eq!(snode.is_config(), true);
    assert_eq!(snode.is_state(), false);
    assert_eq!(snode.is_mandatory(), false);
    assert_eq!(snode.is_keyless_list(), false);
    assert_eq!(snode.is_user_ordered(), false);
    assert_eq!(snode.min_elements(), None);
    assert_eq!(snode.max_elements(), None);
    assert!(snode.musts().unwrap().count() == 0);
    assert!(snode.whens().count() == 0);
    assert!(snode.actions().count() == 0);
    assert!(snode.notifications().count() == 0);
    assert_eq!(snode.is_status_current(), true);
    assert_eq!(snode.is_status_deprecated(), false);
    assert_eq!(snode.is_status_obsolete(), false);

    let snode = ctx
        .find_path("/ietf-interfaces:interfaces-state/interface")
        .expect("Failed to lookup schema node");
    assert_eq!(snode.kind(), SchemaNodeKind::List);
    assert_eq!(snode.name(), "interface");
    assert!(snode.description().is_some());
    assert!(snode.reference().is_none());
    assert_eq!(snode.is_config(), false);
    assert_eq!(snode.is_state(), true);
    assert_eq!(snode.is_mandatory(), false);
    assert_eq!(snode.is_keyless_list(), false);
    // TODO: this is wrong, report back to upstream.
    assert_eq!(snode.is_user_ordered(), true);
    assert_eq!(snode.min_elements(), None);
    assert_eq!(snode.max_elements(), None);
    assert!(snode.musts().unwrap().count() == 0);
    assert!(snode.whens().count() == 0);
    assert!(snode.actions().count() == 0);
    assert!(snode.notifications().count() == 0);
    assert_eq!(snode.is_status_current(), false);
    assert_eq!(snode.is_status_deprecated(), true);
    assert_eq!(snode.is_status_obsolete(), false);
}
