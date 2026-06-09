///
/// Album represents an individual Album in a list.
/// Only one of each album can exist--i.e., no two albums share the
/// same numeric id.
///
#[derive(Clone, Debug, Hash, PartialEq)]
pub struct Album {
    /// Unique numeric id
    id: u64,

    /// name of album
    album_name: String,

    //name of artist
    artist_name: String,
}

impl Default for Album {
    fn default() -> Self {
        Self::new(0, "Default".to_string(), "Deafult".to_string())
    }
}

impl Album {
    /// Create an Album with a specified and name.
    ///
    /// # Arguments
    ///
    /// * `nme` - desired name
    ///
    pub fn new(id: u64, album: String, artist: String) -> Self {
        Self { id, album_name: album, artist_name: artist }
    }

    ///
    /// Retrieve id
    ///
    pub fn get_id(&self) -> u64 {
        self.id
    }

    ///
    /// Update id.
    ///
    /// # Arguments
    ///
    /// * `nme` - replacement id
    ///
    pub fn set_id(&mut self, id: u64) {
        self.id = id;
    }

    ///
    /// Retrieve album name
    ///
    pub fn get_album_name(&self) -> &str {
        &self.album_name;
    }

    ///
    /// Update album name.
    ///
    /// # Arguments
    ///
    /// * `album` - replacement name
    ///
    pub fn set_album_name(mut self, album: String) {
        self.album_name = album;
    }

    ///
    /// Retrieve artist name
    ///
    pub fn get_artist_name(&self) -> &str {
        &self.artist_name;
    }

    ///
    /// Update artist name.
    ///
    /// # Arguments
    ///
    /// * `artist` - replacement name
    ///
    pub fn set_artist_name(mut self, artist: String) {
        self.artist_name = artist;
    }
}

impl std::fmt::Display for Album {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
       writeln!(f, "{}", "{}", self.album, self.artist)?;

        Ok(())
    }
}

///
/// A Homogeneous--i.e., uniform--stack of Items.
///
#[derive(Clone, Debug, Hash, PartialEq)]
pub struct AlbumList {
    ///
    /// The specific type of item out of which this stack is built.
    ///
    cd: Album,

    ///
    /// Represents the number of items in this stack.
    ///
    quantity: usize,
}

impl Default for AlbumList {
    ///
    /// Create an empty stack composed of Air.
    ///
    fn default() -> Self {
        Self::new(Album::default(), 0)
    }
}

/*impl ItemStack {
    ///
    /// Create a stack of the desired type.
    ///
    /// # Arguments
    ///
    /// * `base` - Item out of which the stack is composed
    ///
    /// * `qty` - number of items to place in the stack
    ///
    pub fn new(base: Item, qty: usize) -> Self {
        Self {
            cd: base,
            quantity: qty,
        }
    }

    ///
    /// Retrieve the Item out of which the stack is composed.
    ///
    /// # Returns
    ///
    /// the item that serves as the base
    ///
    pub fn get_album(&self) -> &Album {
        &self.item
    }

    ///
    /// Retrieve the size of the stack.
    ///
    /// # Returns
    ///
    /// the current number of items
    ///
    pub fn size(&self) -> usize {
        self.quantity
    }

    ///
    /// Increase the size of the stack.
    ///
    /// # Arguments
    ///
    /// * `qty` - number of items to add
    ///
    pub fn add_items(&mut self, qty: usize) {
        self.quantity += qty;
    }

    ///
    /// Does the Item contained in this stack permit stacking?
    ///
    /// This can be less formally phrased, is this a stackable ItemStack?
    ///
    /// # Returns
    ///
    /// true if the addition of items is permitted
    ///
    pub fn permits_stacking(&self) -> bool {
        // For now... no albums are stackable
        return true;
    }
}

impl std::fmt::Display for ItemStack {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "({:2}) {}", self.quantity, self.item)?;

        Ok(())
    }
}
    */
