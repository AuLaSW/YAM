use std::ops::Deref;
use indradb;

pub struct Database<D: indradb::Datastore> {
    pub db: indradb::Database<D>
}

impl<D: indradb::Datastore> Deref for Database<D> {
    type Target = indradb::Database<D>;

    fn deref(&self) -> &Self::Target {
        &self.db
    }
}

impl<D: indradb::Datastore> Database<D> {
    pub fn create_vertex_with_properties(&self, vertex: indradb::VertexProperties) -> Result<(), indradb::Error> {
        let uuid = vertex.vertex.id;
        let mut insert_items: Vec<indradb::BulkInsertItem> = vec![
            indradb::BulkInsertItem::Vertex(vertex.vertex)
        ];

        for prop in vertex.props {
            insert_items.push(
                indradb::BulkInsertItem::VertexProperty(
                    uuid,
                    prop.name,
                    prop.value,
                    
                )
            )
        }

        self.db.bulk_insert(insert_items)
    }

    pub fn create_edge_with_properties(&self, edge: indradb::EdgeProperties) -> Result<(), indradb::Error> {
        let _edge = &edge.edge;
        let mut insert_items: Vec<indradb::BulkInsertItem> = vec![
            indradb::BulkInsertItem::Edge(_edge.clone())
        ];

        for prop in edge.props {
            insert_items.push(
                indradb::BulkInsertItem::EdgeProperty(
                    _edge.clone(),
                    prop.name,
                    prop.value,
                    
                )
            )
        }

        self.db.bulk_insert(insert_items)
    }
}
