use super::*;

#[test]
fn test_unlinking_unlinked() {
    let data = init();
    
    assert!(add_subnet(data, 0));
    let comp = add_component(data, ComponentId::Buffer);
    
    assert!(!unlink(data, comp, 0, 0));
    
    exit(data);
}

#[test]
fn test_removing_all_links() {
    let data = init();
    
    assert!(add_subnet(data, 1));
    assert!(add_subnet(data, 2));
    assert!(add_subnet(data, 3));
    
    let id = add_component(data, ComponentId::And);
    
    assert!(link(data, id, 0, 1));
    assert!(link(data, id, 1, 2));
    assert!(link(data, id, 2, 3));
    
    assert!(unlink(data, id, 0, 1));
    assert!(unlink(data, id, 1, 2));
    assert!(unlink(data, id, 2, 3));
    
    exit(data);
}