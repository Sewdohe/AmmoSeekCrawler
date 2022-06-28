use thirtyfour::prelude::*;
use tokio;

struct AmmoSpec {
    retailer: String,
    caliber: String,
    grain: String,
    case: String,
    price: String,
    rounds: String,
    cpr: String,
}

#[tokio::main]
async fn main() -> WebDriverResult<()> {
    let caps = DesiredCapabilities::firefox();
    let driver = WebDriver::new("http://localhost:6565", caps).await?;
    let mut data: Vec<AmmoSpec> = Vec::new();

    // Navigate to ammo seek
    driver
        .get("https://ammoseek.com/ammo/7.62x39mm?co=new&ca=brass")
        .await?;


    // wait until the "ammo" table loads on the page.
    // the await? keyword at the end of the lines causes rust to wait for the code to return before
    // we proceed further - this way we are always sure the elements exist.
    let elem_ammo_table = driver.find_element(By::Id("ammo")).await?;
    let elem_trows = elem_ammo_table.find_elements(By::Tag("tr")).await?;

    // checking how many rows we found in the "ammo" table
    println!("We found {} rows", elem_trows.iter().len());

    for (index, elem) in elem_trows.iter().enumerate() {
        // get the elements data relative to the table row that we are iterating over
        let mut retailer = elem_trows[index].find_element(By::Id("retailerField")).await?;
        retailer = retailer.find_element(By::Tag("span")).await?;
        let mut caliber = elem_trows[index].find_element(By::Id("caliberField")).await?;
        caliber = caliber.find_element(By::Tag("h2")).await?;
        let grain = elem_trows[index].find_element(By::Id("grainField")).await?;
        let mut case = elem_trows[index].find_element(By::Id("caseField")).await?;
        case = case.find_element(By::Tag("span")).await?;
        let price = elem_trows[index].find_element(By::Id("priceField")).await?;
        let rounds = elem_trows[index].find_element(By::Id("roundsField")).await?;
        let cpr = elem_trows[index].find_element(By::Id("cprField")).await?;

        let spec = AmmoSpec { // create a struct based off of the table row data
            retailer: retailer.inner_html().await?,
            caliber: caliber.inner_html().await?,
            case: case.inner_html().await?,
            cpr: cpr.inner_html().await?,
            grain: grain.inner_html().await?,
            price: price.inner_html().await?,
            rounds: rounds.inner_html().await?,
        };

        println!("{} from {} costs {} \n", spec.caliber, spec.retailer, spec.price);
        ()
    }

    // Always explicitly close the browser. There are no async destructors.
    // driver.quit().await?;
    // not closing right now to be able to see the data

    Ok(())
}