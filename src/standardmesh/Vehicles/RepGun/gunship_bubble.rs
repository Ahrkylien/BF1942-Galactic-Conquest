subshader "gunship_bubble_Material0" "StandardMesh/Default"
{
	lighting true;
	materialDiffuse 1 1 1;
	lightingSpecular false;
	materialSpecular 0.8 0.8 0.8;
	materialSpecularPower 4.0;
	envmap true;
	texture "texture/vehicles/repgun/gunship_bubble";
}

subshader "gunship_bubble_Material1" "StandardMesh/Default"
{
	lighting true;
	lightingSpecular false;
      materialSpecular 0.8 0.8 0.8;
	materialSpecularPower 4.0;
	envmap true;
	transparent true;
      twosided true;
	materialDiffuse 1 1 1;
      texture "texture/vehicles/amph/window";
}

