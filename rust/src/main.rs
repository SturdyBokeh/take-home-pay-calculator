fn main() {
    // Hard code in user salary, pension, other taxable, student loans and marriage allowance
    let salary: f32 = 65000.0;
    let pension_rate: f32 = 0.1;
    let other_taxable: f32 = 0.0;
    let monthly_salary_sacrifice: f32 = 200.32;
    let have_undergrad_loan: bool = true;
    let have_postgrad_loan: bool = false;
    let have_marriage_tax_allowance = false;

    // Thresholds and Rates for Tax and Loans etc
    const MARRIAGE_TAX_ALLOWANCE: f32 = 1260.0; // 22/23 marriage allowance (make 0 if not applicable)
    const LOW_TAX_THRESHOLD: f32 = 12570.0; // 22/23 20% tax threshold
    const UPPER_TAX_THRESHOLD: f32 = 50271.0; // 22/23 40% tax threshold
    const UNDERGRAD_LOAN_THRESHOLD: f32 = 27295.0; // 22/23 student loan threshold plan 2
    const UNDERGRAD_LOAN_RATE: f32 = 0.09; // 22/23 student loan rate plan 2
    const POSTGRAD_LOAN_THRESHOLD: f32 = 21000.0;
    const POSTGRAD_LOAN_RATE: f32 = 0.06;
    const NI_STANDARD_RATE: f32 = 0.12; // 22/23 NI standard rate
    const NI_UEL_RATE: f32 = 0.02; // 22/23 NI upper earnings rate
    const NI_THRESHOLD: f32 = 12576.0; // 22/23 NI standard threshold
    const NI_UEL_THRESHOLD: f32 = 50268.0; // 22/23 NI upper earnings threshold

    let gross = gross_income(
        salary,
        pension_rate,
        other_taxable,
        monthly_salary_sacrifice,
    );
    let tax = tax(gross, have_marriage_tax_allowance);
    let ni = ni(gross);
    let (undergrad, postgrad) = student_loans(gross, have_undergrad_loan, have_postgrad_loan);
    let net = gross - tax - ni - undergrad - postgrad;

    println!(
        "Gross: £{}\nTax: £{}\nNI: £{}\nUndergrad: £{}\nPostgrad: £{}\nNet: {}\nNet (monthly): {}",
        gross,
        tax,
        ni,
        undergrad,
        postgrad,
        net,
        net / 12.0
    );

    // Calculate gross income (income - pension)
    fn gross_income(
        salary: f32,
        pension_rate: f32,
        other_taxable: f32,
        salary_sacrifice: f32,
    ) -> f32 {
        let pension: f32 = salary * pension_rate;
        let gross: f32 = salary + other_taxable - pension - 12.0 * salary_sacrifice;
        gross
    }

    // Calculate income Tax
    fn tax(gross: f32, have_marriage_tax_allowance: bool) -> f32 {
        // Check if marriage tax allowance is applied
        let marriage_tax_allowance: f32;
        if have_marriage_tax_allowance == true {
            marriage_tax_allowance = MARRIAGE_TAX_ALLOWANCE;
        } else {
            marriage_tax_allowance = 0.0;
        };

        // Tax will default to 0 (expected for user earning below tax threshold)
        let mut tax: f32 = 0.0;

        // Add on tax for earnings above lower threshold at 20%
        if gross > UPPER_TAX_THRESHOLD {
            tax += (gross - UPPER_TAX_THRESHOLD) * 0.4;
        };

        // Add on tax for earnings above higher threshold at 40%
        if gross > LOW_TAX_THRESHOLD + marriage_tax_allowance {
            tax += (UPPER_TAX_THRESHOLD - LOW_TAX_THRESHOLD - marriage_tax_allowance) * 0.2;
        };
        tax
    }

    // Calulate National Insurance
    fn ni(gross: f32) -> f32 {
        if gross > NI_UEL_THRESHOLD {
            (gross - NI_UEL_THRESHOLD) * NI_UEL_RATE
                + (NI_UEL_THRESHOLD - NI_THRESHOLD) * NI_STANDARD_RATE
        } else if gross > NI_THRESHOLD {
            (gross - NI_THRESHOLD) * NI_STANDARD_RATE
        } else {
            0.0
        }
    }

    // Function to calulate Undergrad and Postgrad loans if the user has them
    fn student_loans(gross: f32, have_ugl: bool, have_pgl: bool) -> (f32, f32) {
        let undergrad_loan: f32;
        let postgrad_loan: f32;

        //If the user has an undergrad loan, calculate the loan payments
        if have_ugl == true {
            undergrad_loan = (gross - UNDERGRAD_LOAN_THRESHOLD) * UNDERGRAD_LOAN_RATE;
        } else {
            undergrad_loan = 0.0;
        };

        //If the user has a postgrad loan, , calculate the loan payments
        if have_pgl == true {
            postgrad_loan = (gross - POSTGRAD_LOAN_THRESHOLD) * POSTGRAD_LOAN_RATE;
        } else {
            postgrad_loan = 0.0;
        };

        (undergrad_loan, postgrad_loan)
    }
}
