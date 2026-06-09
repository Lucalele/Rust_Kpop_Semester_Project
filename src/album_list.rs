use std::collections::LinkedList;

use crate::items::ItemStack;

///
/// An Inventory is composed of n slots. Each slot may store only
/// one type of item-- specified by *slots*.
///
/// Once all slots are filled, no additional Item types may be
/// stored. Individual slots may contain any number of the same
/// Item-- if the Item is stackable.
///
#[derive(Clone, Debug, Hash, PartialEq)]
pub struct Inventory {
    ///
    /// Individual item slots-- each ItemStack occupies one slot.
    ///
    slots: LinkedList<ItemStack>,

    ///
    /// Total number of distinct Item types that can be stored.
    ///
    capacity: usize,
}

impl Default for Inventory {
    fn default() -> Self {
        Self::new(Self::DEFAULT_SIZE)
    }
}

impl Inventory {
    ///
    /// This is the Default Inventory size.
    ///
    const DEFAULT_SIZE: usize = 10;

    ///
    /// This is utility function that takes two ItemStacks and adds the
    /// number of items in the right- hand side stack to the left-hand side stack.
    ///
    /// # Arguments
    ///
    /// * `lhs` - stack whose size will be increased
    ///
    /// * `rhs` - stack whose size we need to examine
    ///
    pub fn merge_stacks(lhs: &mut ItemStack, rhs: ItemStack) {
        // lhs needs to have items added to it.
        // rhs's size is needed
        // lhs.????(rhs.????)
        lhs.add_items(rhs.size());
    }

    ///
    /// Create an inventory with n slots.
    ///
    /// # Arguments
    ///
    /// * `desiredCapacity` - size of the new Inventory
    ///
    pub fn new(desired_capacity: usize) -> Self {
        Self {
            slots: LinkedList::new(),
            capacity: desired_capacity,
        }
    }

    ///
    /// Determine the number of slots currently in use.
    ///
    pub fn utilized_slots(&self) -> usize {
        return self.slots.len();
    }

    ///
    /// Determine the number of empty (unused) slots.
    ///
    pub fn empty_slots(&self) -> usize {
        self.total_slots() - self.utilized_slots()
    }

    ///
    /// Retrieve the capacity (number of distinct types of items) that self
    /// inventory can store.
    ///
    pub fn total_slots(&self) -> usize {
        self.capacity
    }

    ///
    /// Determine if the inventory is considered full.
    ///
    /// # Returns
    ///
    /// true if the current size is equal to capacity
    ///
    pub fn is_full(&self) -> bool {
        self.empty_slots() == 0
    }

    ///
    /// Determine if the inventory is empty.
    ///
    /// # Returns
    ///
    /// true if current size is zero
    ///
    pub fn is_empty(&self) -> bool {
        self.slots.len() == 0
    }

    ///
    /// Search through all slots (Nodes in the LinkedList) and look for a
    /// matching ItemStack.
    ///
    /// # Arguments
    ///
    /// * `key` - stack for which the search is being conducted
    ///
    /// # Returns
    ///
    /// matching stack if one was found and `null` otherwise
    ///
    pub fn find_matching_item_stack(&mut self, key: &ItemStack) -> Option<&mut ItemStack> {
        self.slots.iter_mut().find(|stack| stack == &key)
    }

    ///
    /// This is the standard Linked List append operation from Review 01
    ///
    /// # Arguments
    ///
    /// * `to_add` - data that we want to store in a Node and add to the list
    ///
    pub fn add_item_stack_no_check(&mut self, to_add: ItemStack) {
        self.slots.push_back(to_add);
    }

    ///
    /// Add one or more items to the inventory list.
    ///
    /// # Arguments
    ///
    /// * `stack` - new stack of items to add
    ///
    /// # Returns
    ///
    /// true if *stack* was added and false otherwise
    ///
    pub fn add_items(&mut self, stack: ItemStack) -> bool {
        if let Some(ref mut the_match) = self.find_matching_item_stack(&stack) {
            // If the Item is stackable, add it to the ItemStack
            if the_match.permits_stacking() {
                the_match.add_items(stack.size());

                return true;
            }
        }

        if self.utilized_slots() < self.capacity {
            self.add_item_stack_no_check(stack);
            return true;
        }

        return false;
    }
}

impl std::fmt::Display for Inventory {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(
            f,
            " -Used {} of {} slots",
            self.utilized_slots(),
            self.capacity
        )?;

        for stack in self.slots.iter() {
            writeln!(f, "  {}", stack)?;
        }

        Ok(())
    }
}
