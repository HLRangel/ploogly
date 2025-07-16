![The text "Ploogly", overlaid on the image of a canary](https://github.com/HLRangel/HLRangel/blob/main/plooglylogo.png?raw=true)


Ploogly is a program which some would dare call a static site generator. It was the first project I ever programmed in Rust, and is very unpolished, making use of some bad practices and generally not being nice to look at. Rely on it at your own risk!

I personally like to use it as a little tool to add things to once I need them, because it feels nicer than typing up a few sins with JavaScript every time I want to quickly make a page.

## Installation

Install the latest stable version of the Rust tools with `rustup`. Once you have that, along with an internet connection, run:

```
git clone https://github.com/HLRangel/ploogly
cd ploogly
cargo install --path .
```

And done!

There are plans to make the program installable from source without an internet connection via gracefully handling unavailable dependencies. Eventually...

## How to Use

For now, a simple cheat sheet is available [here](https://github.com/HLRangel/HLRangel/blob/40b4d7471fda7798ae7d30ad6d045519560b354f/plooglycheatsheet.pdf). It was written for version 0.4.5, but it should remain good for a while, given I know a couple people who are relying on the program and wouldn't like to upset them by breaking existing syntax.

## Contributing

Everyone who contributes to Ploogly, by nature of submitting their contribution, agrees to release it under the Mozilla Public License, version 2.0, AND the MIT License, in compliance with the license exception mentioned below.

**Beware!** To release source-code under the MPL, you must include the following header at the top of any files you create (SPDX identifier added for convenience):

```
// SPDX-License-Identifier: MPL-2.0

/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
*/
```

All MPL code tagged under `Exhibit B - "Incompatible With Secondary Licenses"` will be rejected.

You also agree to the frustration dealing with my code will bring upon you :p

## License and exception

All relevant files in the Ploogly distribution are under the terms of the Mozilla Public License, version 2.0. A copy of the terms can be found in the `LICENSE.md` file or at the web address `http://mozilla.org/MPL/2.0/`.

All notorious contributors to and rightsholders of Ploogly's dependencies and Ploogly itself have the right to redistribute the versions of Ploogly which include those contributions/dependencies under the MIT License. A copy of the terms and the list of projects can be found at `special/RH-LICENSE-EXCEPTION.txt`.The license itself can also be found at the web address `https://opensource.org/license/mit`.

THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.