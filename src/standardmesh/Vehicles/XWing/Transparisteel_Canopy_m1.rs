subshader "Transparisteel_Canopy_m1_Material0" "StandardMesh/Default"
{
	lighting true;
	lightingSpecular true;
	materialDiffuse 0.588235 0.588235 0.588235;
	materialSpecular 0.8 0.8 0.8;
	materialSpecularPower 4.0;
	envmap true;
	texture "texture/Vehicles/XWing/XWing_Fus";
}

subshader "Transparisteel_Canopy_m1_Material1" "StandardMesh/Default"
{
	lighting true;
	lightingSpecular false;
	materialDiffuse 1.0 1.0 1.0;
	transparent true;
        envmap False;
        depthWrite false;
	texture "texture/Vehicles/XWing/xwing_glass";
}

