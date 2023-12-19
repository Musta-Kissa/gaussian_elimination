use std::ops::{Deref,DerefMut};

use my_arr_math::MyArr;

struct MyMat<const WIDTH: usize, const HEIGHT: usize> {
    data: [MyArr<i64, WIDTH>; HEIGHT],
}
impl<const WIDTH: usize, const HEIGHT: usize> MyMat<WIDTH, HEIGHT> {
    fn new() -> Self {
        Self {
            data: [MyArr::new([0; WIDTH]); HEIGHT],
        }
    }
    fn solve(&mut self) {
        let mut vec: Vec<i64> = Vec::new();
        for z in 0..HEIGHT {
            vec.clear();
            //get a column to a vec as a buffer so the values dont change when multiplying
            for i in 0..HEIGHT {
                vec.push(self[i][z]);
            }
            // equalizing columns
            // |6| 15 52 25
            // |6|-52  6 40
            // |6| 20 -8 -2
            // first the 0th index then 1st then 2nd till (HEIGHT-1)th (the last column are the values) (achieved with z)
            // *better solution is to multiply by fractions TODO"
            for i in z..HEIGHT {
                for j in z..HEIGHT {
                    // dont multiply by itself
                    //  1 2 3 21 | *2 *3   !*1
                    //  2 3 1 32 | *1 *3   !*2
                    //  3 2 1 32 | *1 *2   !*3
                    if i != j {
                        self[i] = self[i] * vec[j];
                    }
                }
            }
            //now the column is all the same values
            //subtracting rows
            for i in 1..(HEIGHT - z) {
                //from the bottom up so 3-2 | 2-1 | 1-0
                self[HEIGHT - i] = self[HEIGHT - i] - self[HEIGHT - i - 1];
            }
            //now (WIDTH-1) values in the column are 0
            // |6| 15 52 25
            // |0|-52  6 40
            // |0| 20 -8 -2
        }
        //befor devision:[[6,   9,    3,   165] },
        //                [0, -70,   10,  -550] },
        //                [0,   0, -108, -1620] }]
        // assume HEIGHT = 4 for comments
        for z in 0..HEIGHT - 1 {
            //0 1 2
            let curr = HEIGHT - z - 1; // 3 2 1
            let value = WIDTH - 1;
            self[curr] = self[curr] / self[curr][curr]; // devide row so [curr][curr] = 1 and is the anwser
            for x in 0..curr {
                //0 1 2          x = 0        x = 1        x = 2
                // x x x x x    x x x O x    x x O o x    x O o o x
                // o x x x x => o x x O x => o x O o x => o x o o x
                // o o x x x => o o x O x => o o x o x => o o x o x
                // o o o x x    o o o x x    o o o x x    o o o x x
                self[x][value] -= self[curr][value] * self[x][curr];
                self[x][curr] = 0;
            }
        }
        //self[0] /= self[0][0]; //possible TODO; brig this step into the loop above
        self[0] = self[0] / self[0][0];
    }
    fn print2d(&self) {
        for i in 0..self.len() {
            println!("{}", self[i]);
        }
    }
}
impl<const WIDTH: usize, const HEIGHT: usize> Deref for MyMat<WIDTH, HEIGHT> {
    type Target = [MyArr<i64, WIDTH>; HEIGHT];

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
impl<const WIDTH: usize, const HEIGHT: usize> DerefMut for MyMat<WIDTH, HEIGHT> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn modulo(a: i64, b: i64) -> usize {
        (((a % b) + b) % b) as usize
    }

    fn new_test<const WIDTH: usize, const HEIGHT: usize>(
        test_vals: Vec<i64>,
    ) -> MyMat<WIDTH, HEIGHT> {
        assert_eq!( WIDTH, HEIGHT + 1, "const WIDTH has to eq HEIGHT+1, so WIDTH shoud be {}, WIDTH:{},HEIGHT:{}", HEIGHT + 1, WIDTH, HEIGHT);

        let VALUE: usize = HEIGHT;
        let mut mat: [MyArr<i64, WIDTH>; HEIGHT] = [MyArr::new([0; WIDTH]); HEIGHT];

        let mut row: [i64; WIDTH] = [0; WIDTH];
        for j in 0..HEIGHT {
            row[VALUE] = 0; //reset the sum
            for i in 0..HEIGHT {
                // idex is used to vary the values
                //for HEIGHT=4 index: 2 3 4 1 | 3 4 1 2 | 4 1 2 3 | 1 2 3 4
                let index: usize = modulo((i + j + 1) as i64, HEIGHT as i64) + 1;
                row[i] = (index) as i64;
            }
            for i in 0..HEIGHT {
                //calculate sum of elements
                row[VALUE] += row[i] * test_vals[i];
            }
            mat[j] = MyArr::new(row); //Add the row to the matrix
                                      //println!("sum:{}",row[HEIGHT]);
        }
        MyMat { data: mat }
    }

    const HEIGHT: usize = 4;
    const WIDTH: usize = HEIGHT + 1;

    #[test]
    fn it_works() {
        let test_vals: Vec<i64> = vec![30, 11, -5, 4, 2, 9];
        let mut test = new_test::<WIDTH, HEIGHT>(test_vals.clone());
        test.solve();
        test.print2d();

        for i in 0..test.len() {
            assert_eq!(*test[i].last().unwrap(), test_vals[i]);
        }
    }
    #[test]
    fn manual() {
        let test_vals = [5,10,15];
        let mut test = MyMat {
            data: [
                MyArr::new([2, 3, 1, 55]),
                MyArr::new([3, 1, 2, 55]),
                MyArr::new([1, 2, 3, 70]),
            ],
        };
        test.solve();
        test.print2d();
        for i in 0..test.len() {
            assert_eq!(*test[i].last().unwrap(), test_vals[i]);
        }
    }
}

