// Function to generate a Sudoku grid
const generateSudokuGrid = () => {
    const gridContainer = raw.div({ display: "grid", gridTemplateColumns: "repeat(9, 36px)", gap: "2px" });

    for (let i = 0; i < 81; i++) {
        const cell = raw.input(
            { type: "text",maxLength: "1"},
            {
                width: "30px",
                height: "30px",
                textAlign: "center",
                fontSize: "16px",
                border: "1px solid #999",
                outline: "none",
                backgroundColor: "#fff"
            },
            // raw.on((e) => console.log(e))
        );
        gridContainer.append(cell);
    }

    return gridContainer;
};

// Append the Sudoku grid to the body
document.body.append(
    generateSudokuGrid()
);
